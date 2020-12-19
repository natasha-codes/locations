//
//  AuthSession.swift
//  Sonar
//
//  Created by Sasha Weiss on 12/18/20.
//

import AppAuth
import Combine
import Foundation
import SwiftUI

class AuthSession: ObservableObject {
    typealias Token = String

    @Published var hasAuthenticated: Bool = false

    private var authState: OIDAuthState? = nil

    func setAuthState(oidAuthState authState: OIDAuthState) {
        self.authState = authState
        self.hasAuthenticated = true
    }

    func doWithAuth(action: @escaping (_ token: Result<Token, AuthError>) -> Void) {
        guard self.hasAuthenticated, let authState = self.authState else {
            action(.failure(.notAuthenticated))
            return
        }

        authState.performAction { _accessToken, idToken, err in
            if let idToken = idToken {
                action(.success(idToken))
            } else if let err = err, let oidError = OIDErrorCode(rawValue: (err as NSError).code) {
                action(.failure(.openid(code: oidError)))
            } else {
                action(.failure(.unknown))
            }
        }
    }
}

enum AuthError: Error {
    case notAuthenticated
    case openid(code: OIDErrorCode)
    case unknown
}
