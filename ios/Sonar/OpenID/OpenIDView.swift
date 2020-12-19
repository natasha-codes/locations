//
//  OpenIDView.swift
//  Sonar
//
//  Created by Sasha Weiss on 12/18/20.
//

import AppAuth
import SwiftUI
import UIKit

/**
 A `View` that wraps an OpenID Connect (OIDC) flow to authenticate a user, via the
 authority described by`Authority`.

 OIDC flow details: https://rograce.github.io/openid-connect-documentation/explore_auth_code_flow
 */
struct OpenIDView<Authority: OpenIDAuthority>: View {
    // Dumb, but necessary so we can keep `OpenIDView` a struct and
    // be able to assign to a property in a closure without making
    // all the funcs in the chain be `mutating`. Need to keep this
    // a struct for SwiftUI reasons.
    private class OIDAuthSessionWrapper { var wrapped: OIDExternalUserAgentSession? }

    @EnvironmentObject var authSession: AuthSession

    private let inProgressOIDAuthSession = OIDAuthSessionWrapper()

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
                self.authSession.setAuthState(oidAuthState: authState)
            }
        }
    }

    private func getAuthState(presenter: UIViewController, completion: @escaping (Result<OIDAuthState, String>) -> Void) {
        OIDAuthorizationService.discoverConfiguration(forIssuer: Authority.issuer) { configuration, error in
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
            self.inProgressOIDAuthSession.wrapped = OIDAuthState.authState(byPresenting: authRequest,
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
        OpenIDViewController(authorityFriendlyName: Authority.friendlyName, onSignInPressed: self.initiateAuth(presenter:))
    }

    func updateUIViewController(_: OpenIDViewController, context _: Context) {}
}

class OpenIDViewController: UIViewController {
    @IBOutlet var signInButton: UIButton!

    private var authorityFriendlyName: String!
    private var onSignInPressed: ((OpenIDViewController) -> Void)!

    convenience init(authorityFriendlyName: String,
                     onSignInPressed: @escaping (OpenIDViewController) -> Void)
    {
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
