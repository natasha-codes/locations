//
//  SonarApp.swift
//  Sonar
//
//  Created by Sasha Weiss on 12/11/20.
//

import SwiftUI

@main
struct SonarApp: App {
    let authAuthority = MSAOpenIDAuthority()

    @ObservedObject var authSession: OpenIDAuthSession

    init() {
        self.authSession = OpenIDAuthSession(authority: self.authAuthority)
    }

    var body: some Scene {
        WindowGroup {
            if self.authSession.hasAuthenticated {
                TabView {
                    SignedInView()
                        .environmentObject(self.authSession)
                        .tabItem {
                            Text("Main tab")
                        }
                    OpenIDSignOutView(authority: self.authAuthority)
                        .environmentObject(self.authSession)
                        .tabItem {
                            Text("Sign out tab")
                        }
                }
            } else {
                OpenIDSignInView(authority: self.authAuthority)
                    .environmentObject(self.authSession)
            }
        }
    }
}
