//
//  OpenIDAuthSession.swift
//  Sonar
//
//  Created by Sasha Weiss on 12/18/20.
//

import AppAuth
import Combine
import Foundation
import SwiftUI

/**
 Wraps logic related to OpenID Connect-based (OIDC) authentication. Underlying implementation relies on
 the AppAuth OIDC library.

 See an overview of the OIDC flow at: https://rograce.github.io/openid-connect-documentation/explore_auth_code_flow.
 Relevant pieces of code are listed in ordered comments below.
 */
class OpenIDAuthSession<Authority: OpenIDAuthority>: ObservableObject {
    typealias Token = String
    typealias AuthState = (state: OIDAuthState, configuration: OIDServiceConfiguration)
    typealias Result<T> = Swift.Result<T, AuthError>

    @Published var hasAuthenticated: Bool = false

    private var authState: AuthState?
    private var lastRetrievedIDToken: Token?
    private var inProgressOIDAuthSession: OIDExternalUserAgentSession?

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
        // 1. Get the OIDC "discovery document", which is a JSON with metadata about
        //    various OIDC-related endpoints and parameters for the given authority.
        OIDAuthorizationService.discoverConfiguration(forIssuer: Authority.issuer) { configuration, error in
            guard let configuration = configuration else {
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
            let authRequest = OIDAuthorizationRequest(configuration: configuration,
                                                      clientId: Authority.clientId,
                                                      scopes: ["openid"],
                                                      redirectURL: Authority.redirectUri,
                                                      responseType: OIDResponseTypeCode,
                                                      additionalParameters: nil)

            // Take a reference to the auth session here to keep it from dealloc-ing
            self.inProgressOIDAuthSession = OIDAuthState.authState(byPresenting: authRequest,
                                                                   presenting: presenter) { state, error in
                if let state = state {
                    // 3. Store the auth state for later. At this point, the user has authenticated
                    //    with the authority, and using the information in the auth state we should
                    //    be able to, at any point, get a token. (We will do this later, on-demand.)
                    self.authState = (state, configuration)
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
            completion(.failure(.notAuthenticated))
            return
        }

        guard let userAgent = OIDExternalUserAgentIOS(presenting: presenter) else {
            completion(.failure(.unknown))
            return
        }

        // 5. Use the information stored in the auth state fetched during sign-in to
        //    navigate to the authority's `/logout` endpoint, which will log them out.
        let endSessionRequest = OIDEndSessionRequest(configuration: authState.configuration,
                                                     idTokenHint: self.lastRetrievedIDToken ?? "", // Does "" produce a signout error?
                                                     postLogoutRedirectURL: Authority.redirectUri,
                                                     additionalParameters: nil)

        OIDAuthorizationService.present(endSessionRequest, externalUserAgent: userAgent) { _response, error in
            if let error = error {
                if let oidErrorCode = OIDErrorCode(rawValue: (error as NSError).code) {
                    completion(.failure(.openid(code: oidErrorCode)))
                } else {
                    completion(.failure(.unknown))
                }
            } else {
                self.authState = nil
                completion(.success(()))

                // Last, since it notifies subscribers
                self.hasAuthenticated = false
            }
        }
    }
}

enum AuthError: Error {
    case notAuthenticated
    case openid(code: OIDErrorCode)
    case unknown
}
