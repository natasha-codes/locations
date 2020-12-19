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
struct OpenIDView<Authority: OpenIDAuthority>: UIViewControllerRepresentable {
    typealias UIViewControllerType = OpenIDViewController

    @EnvironmentObject var authSession: OpenIDAuthSession<Authority>

    func makeUIViewController(context _: Context) -> OpenIDViewController {
        OpenIDViewController(authorityFriendlyName: Authority.friendlyName, onSignInPressed: { viewController in
            self.authSession.doSignIn(presenter: viewController) { result in
                switch result {
                case .success:
                    print("Sign in successful!")
                case let .failure(error):
                    print("Sign in failed: \(error)")
                }
            }
        })
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
        self.signInButton.setTitle("Sign in with \(self.authorityFriendlyName!)", for: .normal)
        self.signInButton.addTarget(self, action: #selector(self.callOnSignInPressed), for: .touchUpInside)
    }

    @objc private func callOnSignInPressed() {
        self.onSignInPressed?(self)
    }
}
