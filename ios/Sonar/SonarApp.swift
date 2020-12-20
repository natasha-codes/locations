//
//  SonarApp.swift
//  Sonar
//
//  Created by Sasha Weiss on 12/11/20.
//

import SwiftUI

@main
struct SonarApp: App {
    @StateObject var authSession = OpenIDAuthSession(authority: MSAOpenIDAuthority())

    var body: some Scene {
        WindowGroup {
            if self.authSession.hasAuthenticated {
                TabView {
                    SignedInView(apiClient: ApiClient(authSession: self.authSession))
                        .tabItem {
                            Text("Main tab")
                        }
                    OpenIDSignOutView(authority: self.authSession.authority, authSession: self.authSession)
                        .tabItem {
                            Text("Sign out tab")
                        }
                }
            } else {
                OpenIDSignInView(authority: self.authSession.authority, authSession: self.authSession)
            }
        }
    }
}
