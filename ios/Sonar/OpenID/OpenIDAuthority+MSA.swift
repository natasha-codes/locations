//
//  MSAOpenID.swift
//  Sonar
//
//  Created by Sasha Weiss on 12/19/20.
//

import Foundation

struct MSAOpenIDAuthority: OpenIDAuthority {
    let friendlyName = "Microsoft"
    let clientId: String = "97b5900d-bdbe-41bf-8afb-39fdcb0993ee"
    let redirectUri = URL(string: "msauth.com.natasha-codes.sonar://auth/")!
    let issuer = URL(string: "https://login.microsoftonline.com/consumers/v2.0")!
}
