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

class OpenIDAuthSession<Authority: OpenIDAuthority>: ObservableObject {
    typealias Token = String
    typealias AuthState = (state: OIDAuthState, configuration: OIDServiceConfiguration)
    typealias Result<T> = Swift.Result<T, AuthError>

    @Published var hasAuthenticated: Bool = false

    private var inProgressOIDAuthSession: OIDExternalUserAgentSession? = nil
    private var authState: AuthState? = nil

    func setAuthState(oidAuthState authState: AuthState) {
        self.authState = authState
        self.hasAuthenticated = true
    }

    func doWithAuth(action: @escaping (_ token: Result<Token>) -> Void) {
        guard self.hasAuthenticated, let authState = self.authState else {
            action(.failure(.notAuthenticated))
            return
        }

        authState.state.performAction { _accessToken, idToken, err in
            if let idToken = idToken {
                action(.success(idToken))
            } else if let err = err, let oidError = OIDErrorCode(rawValue: (err as NSError).code) {
                action(.failure(.openid(code: oidError)))
            } else {
                action(.failure(.unknown))
            }
        }
    }

    func doSignIn(presenter: UIViewController, completion: @escaping (Result<()>) -> Void) {
        OIDAuthorizationService.discoverConfiguration(forIssuer: Authority.issuer) { configuration, error in
            guard let configuration = configuration else {
                if let error = error, let oidErrorCode = OIDErrorCode(rawValue: (error as NSError).code) {
                    completion(.failure(.openid(code: oidErrorCode)))
                } else {
                    completion(.failure(.unknown))
                }

                return
            }

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
                    self.authState = (state, configuration)
                    self.hasAuthenticated = true
                    completion(.success(()))
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

    func doSignOut(completion: @escaping (Result<()>) -> Void) {
        guard self.hasAuthenticated, let authState = self.authState else {
            completion(.failure(.notAuthenticated))
            return
        }

        /*
        self.authState = nil
        self.hasAuthenticated = false
         */

        /*
        let endSessionRequest = OIDEndSessionRequest(configuration: authState.configuration,
                                                     idTokenHint: <#T##String#>,
                                                     postLogoutRedirectURL: <#T##URL#>,
                                                     additionalParameters: <#T##[String : String]?#>)

        OIDAuthorizationService.present(<#T##request: OIDEndSessionRequest##OIDEndSessionRequest#>,
                                        externalUserAgent: <#T##OIDExternalUserAgent#>,
                                        callback: <#T##OIDEndSessionCallback##OIDEndSessionCallback##(OIDEndSessionResponse?, Error?) -> Void#>)
         */
    }
}

enum AuthError: Error {
    case notAuthenticated
    case openid(code: OIDErrorCode)
    case unknown
}
