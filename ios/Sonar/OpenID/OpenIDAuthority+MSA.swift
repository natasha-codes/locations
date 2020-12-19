//
//  MSAOpenID.swift
//  Sonar
//
//  Created by Sasha Weiss on 12/19/20.
//

import Foundation

typealias MSAOpenIDAuthSession = OpenIDAuthSession<MSAOpenIDAuthority>

struct MSAOpenIDAuthority {}

extension MSAOpenIDAuthority: OpenIDAuthority {
    static let friendlyName = "Microsoft"
    static let clientId: String = "97b5900d-bdbe-41bf-8afb-39fdcb0993ee"
    static let redirectUri = URL(string: "msauth.com.natasha-codes.sonar://auth/")!
    static let issuer = URL(string: "https://login.microsoftonline.com/consumers/v2.0")!
}
