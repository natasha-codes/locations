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
 A `View` designed to interface with the `OpenIDAuthSession` by wrapping a `UIViewController`
 that the `OpenIDAuthSession` can use to perform webview-based actions (e.g., sign in/out).
 */
struct OpenIDView: UIViewControllerRepresentable {
    typealias UIViewControllerType = OpenIDViewController

    private let buttonPrompt: String
    private let onButtonPress: (UIViewController) -> Void

    init(buttonPrompt: String, onButtonPress: @escaping (UIViewController) -> Void) {
        self.buttonPrompt = buttonPrompt
        self.onButtonPress = onButtonPress
    }

    func makeUIViewController(context _: Context) -> OpenIDViewController {
        OpenIDViewController(buttonPrompt: self.buttonPrompt, onButtonPress: self.onButtonPress)
    }

    func updateUIViewController(_: OpenIDViewController, context _: Context) {}
}

class OpenIDViewController: UIViewController {
    @IBOutlet var signInButton: UIButton!

    private var buttonPrompt: String!
    private var onButtonPress: ((OpenIDViewController) -> Void)!

    convenience init(buttonPrompt: String, onButtonPress: @escaping (OpenIDViewController) -> Void) {
        self.init(nibName: "OpenIDView", bundle: nil)

        self.buttonPrompt = buttonPrompt
        self.onButtonPress = onButtonPress
    }

    override func viewDidLoad() {
        self.signInButton.setTitle(self.buttonPrompt, for: .normal)
        self.signInButton.addTarget(self, action: #selector(self.callOnButtonPress), for: .touchUpInside)
    }

    @objc private func callOnButtonPress() {
        self.onButtonPress?(self)
    }
}
