//
//  OpenIDAuthority.swift
//  Sonar
//
//  Created by Sasha Weiss on 12/17/20.
//

import Foundation

protocol OpenIDAuthority {
    static var friendlyName: String { get }
    static var issuer: URL { get }
    static var clientId: String { get }
    static var redirectUri: URL { get }
}
