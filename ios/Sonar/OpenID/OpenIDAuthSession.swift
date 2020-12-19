//
//  OpenIDAuthSession.swift
//  Sonar
//
//  Created by Sasha Weiss on 12/18/20.
//

import AppAuth
import KeychainAccess
import SwiftUI

/**
 Wraps logic related to OpenID Connect-based (OIDC) authentication. Underlying implementation relies on
 the AppAuth OIDC library.

 See an overview of the OIDC flow at: https://rograce.github.io/openid-connect-documentation/explore_auth_code_flow.
 Relevant pieces of code are listed in ordered comments below.
 */
class OpenIDAuthSession: ObservableObject {
    let authority: OpenIDAuthority

    @Published var hasAuthenticated: Bool = false

    private var lastRetrievedIDToken: Token? = nil
    private var inProgressOIDAuthSession: OIDExternalUserAgentSession? = nil

    private let keychain = Keychain(service: "com.natasha-codes.sonar")
    private let kAuthStateDataKey = "OpenIDAuthSession.authState"

    private var authState: AuthState? = nil {
        didSet {
            // Auth state in memory should match Keychain
            do {
                if self.authState == nil {
                    try self.keychain.remove(self.kAuthStateDataKey)
                } else if let newAuthState = self.authState {
                    let serializedAuthState = try NSKeyedArchiver.archivedData(withRootObject: newAuthState, requiringSecureCoding: true)
                    try self.keychain.set(serializedAuthState, key: self.kAuthStateDataKey)
                }
            } catch let e {
                print("Failed to serialize and persist auth state: \(e)")
            }
        }
    }

    init(authority: OpenIDAuthority) {
        self.authority = authority

        do {
            guard let serializedAuthState = try self.keychain.getData(self.kAuthStateDataKey) else {
                return
            }

            // `NSKeyedUnarchiver.unarchiveTopLevelObjectWithData(:)` is deprecated online, in favor
            // of `NSKeyedUnarchiver.unarchivedObject(ofClass:from:)`. However, per this issue:
            // https://github.com/openid/AppAuth-iOS/issues/479 there appears to be a bug in decoding
            // some AppAuth types using this method. Until that's resolved, decode as below.
            if let deserializedObject = try NSKeyedUnarchiver.unarchiveTopLevelObjectWithData(serializedAuthState),
               let authState = deserializedObject as? AuthState
            {
                self.authState = authState
                self.hasAuthenticated = true
            }
        } catch let e {
            print("Failed to deserialize persisted auth state: \(e)")
        }
    }

    func setAuthState(oidAuthState authState: AuthState) {
        self.authState = authState
        self.hasAuthenticated = true
    }

    func doWithAuth(action: @escaping (_ token: Result<Token>) -> Void) {
        guard self.hasAuthenticated, let authState = self.authState else {
            action(.failure(.notAuthenticated))
            return
        }

        // 4. When asked, use the information stored in the auth state fetched
        //    during sign-in to call the authority's `/token` endpoint to get
        //    an `access_token` (not super useful, since we requested no OAuth
        //    scopes) and an `id_token` (since we requested the "openid" scope).
        //    Note that `.performAction` below is a library call.
        authState.state.performAction { _accessToken, idToken, err in
            if let idToken = idToken {
                self.lastRetrievedIDToken = idToken
                action(.success(idToken))
            } else if let err = err, let oidError = OIDErrorCode(rawValue: (err as NSError).code) {
                action(.failure(.openid(code: oidError)))
            } else {
                action(.failure(.unknown))
            }
        }
    }

    func doSignIn(presenter: UIViewController, completion: @escaping (Result<Void>) -> Void) {
        guard !self.hasAuthenticated, self.authState == nil else {
            completion(.success(()))
            return
        }

        // 1. Get the OIDC "discovery document", which is a JSON with metadata about
        //    various OIDC-related endpoints and parameters for the given authority.
        OIDAuthorizationService.discoverConfiguration(forIssuer: self.authority.issuer) { config, error in
            guard let config = config else {
                if let error = error, let oidErrorCode = OIDErrorCode(rawValue: (error as NSError).code) {
                    completion(.failure(.openid(code: oidErrorCode)))
                } else {
                    completion(.failure(.unknown))
                }

                return
            }

            // 2. Using the discovered configuration, construct a request to authenticate a user
            //    (and authorize ourselves) by calling a `/authorize` endpoint. Gets a "code",
            //    which we (well, AppAuth) will later use to get tokens. Passes the "openid"
            //    scope, which means the code we get will later be able to get the `id_token`
            //    that we eventually want (since it *authenticates* a user, vs. an `access_token`
            //    which *authorizes* us to call the authority's APIs on behalf of the user).
            let authRequest = OIDAuthorizationRequest(configuration: config,
                                                      clientId: self.authority.clientId,
                                                      scopes: ["openid"],
                                                      redirectURL: self.authority.redirectUri,
                                                      responseType: OIDResponseTypeCode,
                                                      additionalParameters: nil)

            // Take a reference to the auth session here to keep it from dealloc-ing
            self.inProgressOIDAuthSession = OIDAuthState.authState(byPresenting: authRequest,
                                                                   presenting: presenter) { state, error in
                if let state = state {
                    // 3. Store the auth state for later. At this point, the user has authenticated
                    //    with the authority, and using the information in the auth state we should
                    //    be able to, at any point, get a token. (We will do this later, on-demand.)
                    self.authState = AuthState(state: state, config: config)
                    completion(.success(()))

                    // Last, since it notifies subscribers
                    self.hasAuthenticated = true
                } else {
                    if let error = error, let oidErrorCode = OIDErrorCode(rawValue: (error as NSError).code) {
                        completion(.failure(.openid(code: oidErrorCode)))
                    } else {
                        completion(.failure(.unknown))
                    }
                }
            }
        }
    }

    func doSignOut(presenter: UIViewController, completion: @escaping (Result<Void>) -> Void) {
        guard self.hasAuthenticated, let authState = self.authState else {
            completion(.success(()))
            return
        }

        guard let userAgent = OIDExternalUserAgentIOS(presenting: presenter) else {
            completion(.failure(.unknown))
            return
        }

        // 5. Use the information stored in the auth state fetched during sign-in to
        //    navigate to the authority's `/logout` endpoint, which will log them out.
        let endSessionRequest = OIDEndSessionRequest(configuration: authState.config,
                                                     idTokenHint: self.lastRetrievedIDToken ?? "", // Does "" produce a signout error?
                                                     postLogoutRedirectURL: self.authority.redirectUri,
                                                     additionalParameters: nil)

        let succeed = {
            self.authState = nil
            completion(.success(()))

            // Last, since it notifies subscribers
            self.hasAuthenticated = false
        }

        // Take a reference to the auth session here to keep it from dealloc-ing
        self.inProgressOIDAuthSession = OIDAuthorizationService.present(endSessionRequest, externalUserAgent: userAgent) { _response, error in
            if let error = error {
                if let oidErrorCode = OIDErrorCode(rawValue: (error as NSError).code) {
                    // It seems that (at least the MSA) logout flow does not automatically
                    // redirect into the app and therefore requires a manual closing of the
                    // webview, which reads as a user cancel.
                    //
                    // Note that if the user manually cancels during some interactive stage
                    // of the `/logout` page, we will consider them signed out although there
                    // may be cached credentials in the webview.
                    //
                    // Also note that I think there's a bug in the iOS simulator where clearing
                    // the user's credentials via `/logout` (again, at least for MSA) just doesn't
                    // work. I suspect it's due to a bug in how the simulator shares cookies/state
                    // with the Safari instance, since clearing Safari cookies (which should clear
                    // cookies in the Safari-based secure webview) should definitely clear the
                    // cedentials and that doesn't work either. Try it on device, should work.
                    if case .userCanceledAuthorizationFlow = oidErrorCode {
                        succeed()
                    } else {
                        completion(.failure(.openid(code: oidErrorCode)))
                    }
                } else {
                    completion(.failure(.unknown))
                }
            } else {
                succeed()
            }
        }
    }
}

// MARK: Nested types

extension OpenIDAuthSession {
    typealias Token = String
    typealias Result<T> = Swift.Result<T, Error>

    enum Error: Swift.Error {
        case notAuthenticated
        case openid(code: OIDErrorCode)
        case unknown
    }

    @objc(OpenIDAuthSessionAuthState) class AuthState: NSObject, NSSecureCoding {
        let state: OIDAuthState
        let config: OIDServiceConfiguration

        init(state: OIDAuthState, config: OIDServiceConfiguration) {
            self.state = state
            self.config = config
        }

        static var supportsSecureCoding: Bool { true }

        func encode(with coder: NSCoder) {
            coder.encode(self.state, forKey: "state")
            coder.encode(self.config, forKey: "config")
        }

        required init?(coder: NSCoder) {
            guard let state = coder.decodeObject(of: OIDAuthState.self, forKey: "state"),
                  let config = coder.decodeObject(of: OIDServiceConfiguration.self, forKey: "config")
            else {
                return nil
            }

            self.state = state
            self.config = config
        }
    }
}
