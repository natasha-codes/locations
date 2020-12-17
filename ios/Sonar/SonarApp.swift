//
//  SonarApp.swift
//  Sonar
//
//  Created by Sasha Weiss on 12/11/20.
//

import SwiftUI

@main
struct SonarApp: App {
    var body: some Scene {
        WindowGroup {
            AuthView().onOpenURL(perform: { url in
                print("url: \(url)")
            })
        }
    }
}
