//
//  OpenIDViewController.swift
//  Sonar
//
//  Created by Sasha Weiss on 12/16/20.
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
    private var authState: OIDAuthState?
    private var currentAuthSession: OIDExternalUserAgentSession?

    fileprivate func initiateAuth(presenter: UIViewController) {
        self.performAuthorization(presenter: presenter) { result in
            if case let .failure(err) = result {
                print(err)
                return
            }

            self.authState?.performAction(freshTokens: { _, idToken, error in
                if let idToken = idToken {
                    print("ID token: \(idToken)")
                } else if let error = error {
                    print("Error performing action with fresh tokens: \(error)")
                }
            }, additionalRefreshParameters: nil)
        }
    }

    private func performAuthorization(presenter: UIViewController, completion: @escaping (Result<Void, String>) -> Void) {
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
                                                             presenting: presenter) { [weak self] state, error in
                guard let self = self else {
                    completion(.failure("Dealloced getting auth state"))
                    return
                }

                if let state = state {
                    self.authState = state
                    completion(.success(()))
                } else {
                    self.authState = nil
                    completion(.failure("Error performing auth request: \(error?.localizedDescription ?? "Unknown error")"))
                }
            }
        }
    }
}

extension OpenIDView: UIViewControllerRepresentable {
    typealias UIViewControllerType = OpenIDViewController

    func makeUIViewController(context _: Context) -> OpenIDViewController {
        OpenIDViewController(onButtonPress: { [weak self] presenter in
            guard let self = self else {
                print("Dealloced in button press closure")
                return
            }

            self.initiateAuth(presenter: presenter)
        })
    }

    func updateUIViewController(_: OpenIDViewController, context _: Context) {}
}

class OpenIDViewController: UIViewController {
    @IBOutlet var doTheAuthButton: UIButton!

    private var onButtonPress: ((OpenIDViewController) -> Void)?

    convenience init(onButtonPress: @escaping (OpenIDViewController) -> Void) {
        self.init(nibName: "OpenIDView", bundle: nil)
        self.onButtonPress = onButtonPress
    }

    override func viewDidLoad() {
        self.doTheAuthButton.addTarget(self, action: #selector(self.onButtonPressSelector), for: .touchUpInside)
    }

    @objc private func onButtonPressSelector() {
        self.onButtonPress?(self)
    }
}

extension String: Error {}
