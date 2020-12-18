//
//  OpenIDView.swift
//  Sonar
//
//  Created by Sasha Weiss on 12/18/20.
//

import AppAuth
import Foundation
import SwiftUI
import UIKit

/**
 A `View` that wraps an OpenID Connect (OIDC) flow to authenticate a user, via the
 authority described by`Authority`.

 OIDC flow details: https://rograce.github.io/openid-connect-documentation/explore_auth_code_flow
 */
final class OpenIDView<Authority: OpenIDAuthority> {
    private var currentAuthSession: OIDExternalUserAgentSession?

    @Binding var auth: Auth?

    init(auth: Binding<Auth?>?) {
        self._auth = auth ?? Binding.constant(nil)
    }

    fileprivate func initiateAuth(presenter: UIViewController) {
        self.getAuthState(presenter: presenter) { result in
            if case let .failure(err) = result {
                print(err)
                return
            }

            switch result {
            case let .failure(err):
                print(err)
            case let .success(authState):
                authState.performAction(freshTokens: { _, idToken, error in
                    if let idToken = idToken {
                        print("ID token: \(idToken)")
                    } else if let error = error {
                        print("Error performing action with fresh tokens: \(error)")
                    }
                }, additionalRefreshParameters: nil)
            }
        }
    }

    private func getAuthState(presenter: UIViewController, completion: @escaping (Result<OIDAuthState, String>) -> Void) {
        OIDAuthorizationService.discoverConfiguration(forIssuer: Authority.issuer) { [weak self] configuration, error in
            guard let self = self else {
                completion(.failure("Dealloced discovering configuration"))
                return
            }

            guard let configuration = configuration else {
                completion(.failure("Error retrieving discovery document: \(error?.localizedDescription ?? "Unknown error")"))
                return
            }

            let authRequest = OIDAuthorizationRequest(configuration: configuration,
                                                      clientId: Authority.clientId,
                                                      scopes: ["openid"],
                                                      redirectURL: Authority.redirectUri,
                                                      responseType: OIDResponseTypeCode,
                                                      additionalParameters: nil)

            // Take a reference to the auth session here to keep it from dealloc-ing
            self.currentAuthSession = OIDAuthState.authState(byPresenting: authRequest,
                                                             presenting: presenter) { state, error in
                if let state = state {
                    completion(.success(state))
                } else {
                    completion(.failure("Error performing auth request: \(error?.localizedDescription ?? "Unknown error")"))
                }
            }
        }
    }
}

extension OpenIDView: UIViewControllerRepresentable {
    typealias UIViewControllerType = OpenIDViewController

    func makeUIViewController(context _: Context) -> OpenIDViewController {
        OpenIDViewController(authorityFriendlyName: Authority.friendlyName, onSignInPressed: { [weak self] presenter in
            guard let self = self else {
                return
            }

            self.initiateAuth(presenter: presenter)
        })
    }

    func updateUIViewController(_: OpenIDViewController, context _: Context) {}
}

class OpenIDViewController: UIViewController {
    @IBOutlet var signInButton: UIButton!

    private var authorityFriendlyName: String!
    private var onSignInPressed: ((OpenIDViewController) -> Void)!

    convenience init(authorityFriendlyName: String,
                     onSignInPressed: @escaping (OpenIDViewController) -> Void) {
        self.init(nibName: "OpenIDView", bundle: nil)

        self.authorityFriendlyName = authorityFriendlyName
        self.onSignInPressed = onSignInPressed
    }

    override func viewDidLoad() {
        self.signInButton
            .setTitle("Sign in with \(self.authorityFriendlyName!)",
                      for: .normal)

        self.signInButton.addTarget(self, action: #selector(self.callOnSignInPressed), for: .touchUpInside)
    }

    @objc private func callOnSignInPressed() {
        self.onSignInPressed?(self)
    }
}
