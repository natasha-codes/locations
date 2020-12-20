//
//  ApiOperation.swift
//  Sonar
//
//  Created by Sasha Weiss on 12/19/20.
//

import Alamofire
import Foundation

protocol ApiOperation {
    associatedtype Response: Decodable

    var method: HTTPMethod { get }
    var parameters: Encodable { get }
}
