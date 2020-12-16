//
//  OpenID.swift
//  Locations
//
//  Created by Sasha Weiss on 12/15/20.
//

import Foundation
import AppAuth

func discoverOpenID() {
    let issuer = URL(string: "https://login.microsoftonline.com/consumers/v2.0")!

    OIDAuthorizationService.discoverConfiguration(forIssuer: issuer) { (configuration, error) in
        guard let configuration = configuration else {
            print("Error retrieving discovery document: \(error?.localizedDescription ?? "Unknown error")")
            return
        }

        print("Configuration: \(configuration)")
    }
}
