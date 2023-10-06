/* tslint:disable */
/* eslint-disable */
/**
 * kraken
 * The core component of kraken-project
 *
 * The version of the OpenAPI document: 0.1.0
 * Contact: git@omikron.dev
 *
 * NOTE: This class is auto generated by OpenAPI Generator (https://openapi-generator.tech).
 * https://openapi-generator.tech
 * Do not edit the class manually.
 */

import { exists, mapValues } from '../runtime';
/**
 * Host Alive check request
 * @export
 * @interface HostsAliveRequest
 */
export interface HostsAliveRequest {
    /**
     * 
     * @type {Array<string>}
     * @memberof HostsAliveRequest
     */
    targets: Array<string>;
    /**
     * 
     * @type {number}
     * @memberof HostsAliveRequest
     */
    timeout: number;
    /**
     * 
     * @type {number}
     * @memberof HostsAliveRequest
     */
    concurrentLimit: number;
}

/**
 * Check if a given object implements the HostsAliveRequest interface.
 */
export function instanceOfHostsAliveRequest(value: object): boolean {
    let isInstance = true;
    isInstance = isInstance && "targets" in value;
    isInstance = isInstance && "timeout" in value;
    isInstance = isInstance && "concurrentLimit" in value;

    return isInstance;
}

export function HostsAliveRequestFromJSON(json: any): HostsAliveRequest {
    return HostsAliveRequestFromJSONTyped(json, false);
}

export function HostsAliveRequestFromJSONTyped(json: any, ignoreDiscriminator: boolean): HostsAliveRequest {
    if ((json === undefined) || (json === null)) {
        return json;
    }
    return {
        
        'targets': json['targets'],
        'timeout': json['timeout'],
        'concurrentLimit': json['concurrent_limit'],
    };
}

export function HostsAliveRequestToJSON(value?: HostsAliveRequest | null): any {
    if (value === undefined) {
        return undefined;
    }
    if (value === null) {
        return null;
    }
    return {
        
        'targets': value.targets,
        'timeout': value.timeout,
        'concurrent_limit': value.concurrentLimit,
    };
}

