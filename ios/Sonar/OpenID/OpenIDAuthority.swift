//
//  OpenIDAuthority.swift
//  Sonar
//
//  Created by Sasha Weiss on 12/17/20.
//

import Foundation

protocol OpenIDAuthority {
    var friendlyName: String { get }
    var issuer: URL { get }
    var clientId: String { get }
    var redirectUri: URL { get }
}
