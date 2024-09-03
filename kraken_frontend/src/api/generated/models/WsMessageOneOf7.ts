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
/**
 * A result for hosts alive check
 * @export
 * @interface WsMessageOneOf7
 */
export interface WsMessageOneOf7 {
    /**
     * The corresponding id of the attack
     * @type {string}
     * @memberof WsMessageOneOf7
     */
    attackUuid: string;
    /**
     * A host which could be reached
     * @type {string}
     * @memberof WsMessageOneOf7
     */
    host: string;
    /**
     * 
     * @type {string}
     * @memberof WsMessageOneOf7
     */
    type: WsMessageOneOf7TypeEnum;
}


/**
 * @export
 */
export const WsMessageOneOf7TypeEnum = {
    HostsAliveCheck: 'HostsAliveCheck'
} as const;
export type WsMessageOneOf7TypeEnum = typeof WsMessageOneOf7TypeEnum[keyof typeof WsMessageOneOf7TypeEnum];


/**
 * Check if a given object implements the WsMessageOneOf7 interface.
 */
export function instanceOfWsMessageOneOf7(value: object): boolean {
    let isInstance = true;
    isInstance = isInstance && "attackUuid" in value;
    isInstance = isInstance && "host" in value;
    isInstance = isInstance && "type" in value;

    return isInstance;
}

export function WsMessageOneOf7FromJSON(json: any): WsMessageOneOf7 {
    return WsMessageOneOf7FromJSONTyped(json, false);
}

export function WsMessageOneOf7FromJSONTyped(json: any, ignoreDiscriminator: boolean): WsMessageOneOf7 {
    if ((json === undefined) || (json === null)) {
        return json;
    }
    return {
        
        'attackUuid': json['attack_uuid'],
        'host': json['host'],
        'type': json['type'],
    };
}

export function WsMessageOneOf7ToJSON(value?: WsMessageOneOf7 | null): any {
    if (value === undefined) {
        return undefined;
    }
    if (value === null) {
        return null;
    }
    return {
        
        'attack_uuid': value.attackUuid,
        'host': value.host,
        'type': value.type,
    };
}

