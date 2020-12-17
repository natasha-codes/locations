//
//  OpenIDViewController.swift
//  Locations
//
//  Created by Sasha Weiss on 12/16/20.
//

import AppAuth
import Foundation
import SwiftUI
import UIKit

struct OpenIDView: UIViewControllerRepresentable {
    func makeUIViewController(context _: Context) -> some UIViewController {
        OpenIDViewController()
    }

    func updateUIViewController(_: UIViewControllerType, context _: Context) {}
}

class OpenIDViewController: UIViewController {
    static let kMSAIssuer = URL(string: "https://login.microsoftonline.com/consumers/v2.0")!
    static let kMSAClientID = "97b5900d-bdbe-41bf-8afb-39fdcb0993ee"
    static let kMSARedirectURL = URL(string: "msauth.com.natasha-codes.Locations://auth")!

    @IBOutlet var doTheAuthButton: UIButton!

    var authState: OIDAuthState?

    convenience init() {
        self.init(nibName: "OpenIDView", bundle: nil)
    }

    override func viewDidLoad() {
        self.doTheAuthButton.addTarget(self, action: #selector(self.doTheAuth), for: .touchUpInside)
    }

    @objc private func doTheAuth() {
        self.getAuthToken { result in
            print("\(result)")
        }
    }

    private func getAuthToken(completion: @escaping (Result<String, String>) -> Void) {
        OIDAuthorizationService.discoverConfiguration(forIssuer: OpenIDViewController.kMSAIssuer) { [weak self] configuration, error in
            guard let self = self else {
                completion(.failure("I was dealloc-ed :("))
                return
            }

            guard let configuration = configuration else {
                completion(.failure("Error retrieving discovery document: \(error?.localizedDescription ?? "Unknown error")"))
                return
            }

            let authRequest = OIDAuthorizationRequest(configuration: configuration,
                                                      clientId: OpenIDViewController.kMSAClientID,
                                                      scopes: nil,
                                                      redirectURL: OpenIDViewController.kMSARedirectURL,
                                                      responseType: OIDResponseTypeCode,
                                                      additionalParameters: nil)

            let appDelegateUncasted = UIApplication.shared.delegate!
            let appDelegate = appDelegateUncasted as! AppDelegate

            appDelegate.currentAuthSession = OIDAuthState.authState(byPresenting: authRequest,
                                                                    presenting: self) { [weak self] state, error in
                guard let self = self else {
                    completion(.failure("I was dealloc-ed, part two :("))
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

extension String: Error {}
