//
//  SonarApp.swift
//  Sonar
//
//  Created by Sasha Weiss on 12/11/20.
//

import SwiftUI

@main
struct SonarApp: App {
    @ObservedObject var authSession = MSAOpenIDAuthSession()

    var body: some Scene {
        WindowGroup {
            if self.authSession.hasAuthenticated {
                SignedInView()
                    .environmentObject(self.authSession)
            } else {
                OpenIDSignInView<MSAOpenIDAuthority>()
                    .environmentObject(self.authSession)
            }
        }
    }
}
