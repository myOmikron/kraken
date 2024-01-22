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
import type { SimpleHost } from './SimpleHost';
import {
    SimpleHostFromJSON,
    SimpleHostFromJSONTyped,
    SimpleHostToJSON,
} from './SimpleHost';

/**
 * A new host was found
 * @export
 * @interface WsMessageOneOf14
 */
export interface WsMessageOneOf14 {
    /**
     * The workspace this host is related to
     * @type {string}
     * @memberof WsMessageOneOf14
     */
    workspace: string;
    /**
     * 
     * @type {SimpleHost}
     * @memberof WsMessageOneOf14
     */
    host: SimpleHost;
    /**
     * 
     * @type {string}
     * @memberof WsMessageOneOf14
     */
    type: WsMessageOneOf14TypeEnum;
}


/**
 * @export
 */
export const WsMessageOneOf14TypeEnum = {
    NewHost: 'NewHost'
} as const;
export type WsMessageOneOf14TypeEnum = typeof WsMessageOneOf14TypeEnum[keyof typeof WsMessageOneOf14TypeEnum];


/**
 * Check if a given object implements the WsMessageOneOf14 interface.
 */
export function instanceOfWsMessageOneOf14(value: object): boolean {
    let isInstance = true;
    isInstance = isInstance && "workspace" in value;
    isInstance = isInstance && "host" in value;
    isInstance = isInstance && "type" in value;

    return isInstance;
}

export function WsMessageOneOf14FromJSON(json: any): WsMessageOneOf14 {
    return WsMessageOneOf14FromJSONTyped(json, false);
}

export function WsMessageOneOf14FromJSONTyped(json: any, ignoreDiscriminator: boolean): WsMessageOneOf14 {
    if ((json === undefined) || (json === null)) {
        return json;
    }
    return {
        
        'workspace': json['workspace'],
        'host': SimpleHostFromJSON(json['host']),
        'type': json['type'],
    };
}

export function WsMessageOneOf14ToJSON(value?: WsMessageOneOf14 | null): any {
    if (value === undefined) {
        return undefined;
    }
    if (value === null) {
        return null;
    }
    return {
        
        'workspace': value.workspace,
        'host': SimpleHostToJSON(value.host),
        'type': value.type,
    };
}

