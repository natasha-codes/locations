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
                TabView {
                    SignedInView()
                        .environmentObject(self.authSession)
                        .tabItem {
                            Text("Main tab")
                        }
                    MSAOpenIDSignOutView()
                        .environmentObject(self.authSession)
                        .tabItem {
                            Text("Sign out tab")
                        }
                }
            } else {
                MSAOpenIDSignInView()
                    .environmentObject(self.authSession)
            }
        }
    }
}
