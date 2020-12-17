//
//  LocationsApp.swift
//  Locations
//
//  Created by Sasha Weiss on 12/11/20.
//

import AppAuth
import SwiftUI

@main
struct LocationsApp: App {
    @UIApplicationDelegateAdaptor(AppDelegate.self) var appDelegate

    var body: some Scene {
        WindowGroup {
            ContentView()
                .onOpenURL { url in
                    print("\(url)")
                }
        }
    }
}

class AppDelegate: NSObject, UIApplicationDelegate {
    var currentAuthSession: OIDExternalUserAgentSession?

    func application(_: UIApplication, open url: URL, options _: [UIApplication.OpenURLOptionsKey: Any] = [:]) -> Bool {
        // Resume the auth flow, if this URL is related
        if let authSession = self.currentAuthSession,
           authSession.resumeExternalUserAgentFlow(with: url)
        {
            self.currentAuthSession = nil
            return true
        }

        return false
    }
}
