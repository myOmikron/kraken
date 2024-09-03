/* tslint:disable */
/* eslint-disable */
/**
 * kraken
 * The core component of kraken-project
 *
 * The version of the OpenAPI document: 0.4.2
 * Contact: git@omikron.dev
 *
 * NOTE: This class is auto generated by OpenAPI Generator (https://openapi-generator.tech).
 * https://openapi-generator.tech
 * Do not edit the class manually.
 */

import { exists, mapValues } from '../runtime';
import type { PortOrRange } from './PortOrRange';
import {
    PortOrRangeFromJSON,
    PortOrRangeFromJSONTyped,
    PortOrRangeToJSON,
} from './PortOrRange';

/**
 * The request to start a service detection
 * @export
 * @interface ServiceDetectionRequest
 */
export interface ServiceDetectionRequest {
    /**
     * The leech to use
     * 
     * Leave empty to use a random leech
     * @type {string}
     * @memberof ServiceDetectionRequest
     */
    leechUuid?: string | null;
    /**
     * The ip addresses / networks or domains to scan
     * @type {Array<string>}
     * @memberof ServiceDetectionRequest
     */
    targets: Array<string>;
    /**
     * List of single ports and port ranges
     * 
     * If no values are supplied, 1-65535 is used as default
     * @type {Array<PortOrRange>}
     * @memberof ServiceDetectionRequest
     */
    ports?: Array<PortOrRange>;
    /**
     * The time to wait until a connection is considered failed.
     * 
     * The timeout is specified in milliseconds.
     * @type {number}
     * @memberof ServiceDetectionRequest
     */
    connectTimeout: number;
    /**
     * Time to wait for a response after sending the payload
     * (or after establishing a connection, if not payload is to be sent)
     * 
     * The timeout is specified in milliseconds.
     * @type {number}
     * @memberof ServiceDetectionRequest
     */
    receiveTimeout: number;
    /**
     * The interval that should be wait between retries on a port.
     * 
     * The interval is specified in milliseconds.
     * @type {number}
     * @memberof ServiceDetectionRequest
     */
    retryInterval: number;
    /**
     * The number of times the connection should be retried if it failed.
     * @type {number}
     * @memberof ServiceDetectionRequest
     */
    maxRetries: number;
    /**
     * The concurrent task limit
     * @type {number}
     * @memberof ServiceDetectionRequest
     */
    concurrentLimit: number;
    /**
     * Skips the initial icmp check.
     * 
     * All hosts are assumed to be reachable
     * @type {boolean}
     * @memberof ServiceDetectionRequest
     */
    skipIcmpCheck: boolean;
    /**
     * The workspace to execute the attack in
     * @type {string}
     * @memberof ServiceDetectionRequest
     */
    workspaceUuid: string;
}

/**
 * Check if a given object implements the ServiceDetectionRequest interface.
 */
export function instanceOfServiceDetectionRequest(value: object): boolean {
    let isInstance = true;
    isInstance = isInstance && "targets" in value;
    isInstance = isInstance && "connectTimeout" in value;
    isInstance = isInstance && "receiveTimeout" in value;
    isInstance = isInstance && "retryInterval" in value;
    isInstance = isInstance && "maxRetries" in value;
    isInstance = isInstance && "concurrentLimit" in value;
    isInstance = isInstance && "skipIcmpCheck" in value;
    isInstance = isInstance && "workspaceUuid" in value;

    return isInstance;
}

export function ServiceDetectionRequestFromJSON(json: any): ServiceDetectionRequest {
    return ServiceDetectionRequestFromJSONTyped(json, false);
}

export function ServiceDetectionRequestFromJSONTyped(json: any, ignoreDiscriminator: boolean): ServiceDetectionRequest {
    if ((json === undefined) || (json === null)) {
        return json;
    }
    return {
        
        'leechUuid': !exists(json, 'leech_uuid') ? undefined : json['leech_uuid'],
        'targets': json['targets'],
        'ports': !exists(json, 'ports') ? undefined : ((json['ports'] as Array<any>).map(PortOrRangeFromJSON)),
        'connectTimeout': json['connect_timeout'],
        'receiveTimeout': json['receive_timeout'],
        'retryInterval': json['retry_interval'],
        'maxRetries': json['max_retries'],
        'concurrentLimit': json['concurrent_limit'],
        'skipIcmpCheck': json['skip_icmp_check'],
        'workspaceUuid': json['workspace_uuid'],
    };
}

export function ServiceDetectionRequestToJSON(value?: ServiceDetectionRequest | null): any {
    if (value === undefined) {
        return undefined;
    }
    if (value === null) {
        return null;
    }
    return {
        
        'leech_uuid': value.leechUuid,
        'targets': value.targets,
        'ports': value.ports === undefined ? undefined : ((value.ports as Array<any>).map(PortOrRangeToJSON)),
        'connect_timeout': value.connectTimeout,
        'receive_timeout': value.receiveTimeout,
        'retry_interval': value.retryInterval,
        'max_retries': value.maxRetries,
        'concurrent_limit': value.concurrentLimit,
        'skip_icmp_check': value.skipIcmpCheck,
        'workspace_uuid': value.workspaceUuid,
    };
}

