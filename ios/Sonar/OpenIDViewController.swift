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

final class OpenIDView {
    static let kMSAIssuer = URL(string: "https://login.microsoftonline.com/consumers/v2.0")!
    static let kMSAClientID = "97b5900d-bdbe-41bf-8afb-39fdcb0993ee"
    static let kMSARedirectURL = URL(string: "msauth.com.natasha-codes.sonar://auth")!

    private var authState: OIDAuthState?
    private var currentAuthSession: OIDExternalUserAgentSession?

    func handleAuthUrl(url: URL) {
        if let authSession = self.currentAuthSession, authSession.resumeExternalUserAgentFlow(with: url) {
            self.currentAuthSession = nil
        }
    }

    fileprivate func initiateAuth(presenter: UIViewController) {
        self.getAuthToken(presenter: presenter) { result in
            print("\(result)")
        }
    }

    private func getAuthToken(presenter: UIViewController, completion: @escaping (Result<String, String>) -> Void) {
        OIDAuthorizationService.discoverConfiguration(forIssuer: OpenIDView.kMSAIssuer) { [weak self] configuration, error in
            guard let self = self else {
                completion(.failure("Dealloced discovering configuration"))
                return
            }

            guard let configuration = configuration else {
                completion(.failure("Error retrieving discovery document: \(error?.localizedDescription ?? "Unknown error")"))
                return
            }

            let authRequest = OIDAuthorizationRequest(configuration: configuration,
                                                      clientId: OpenIDView.kMSAClientID,
                                                      scopes: nil,
                                                      redirectURL: OpenIDView.kMSARedirectURL,
                                                      responseType: OIDResponseTypeCode,
                                                      additionalParameters: nil)

            self.currentAuthSession = OIDAuthState.authState(byPresenting: authRequest,
                                                             presenting: presenter) { [weak self] state, error in
                guard let self = self else {
                    completion(.failure("Dealloced getting auth state"))
                    return
                }

                if let state = state {
                    self.authState = state

                    print("ID token: \(self.authState!.lastTokenResponse?.idToken ?? "No ID token?")")
                } else {
                    completion(.failure("Error performing auth request: \(error?.localizedDescription ?? "Unknown error")"))
                    return
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
