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
 * A http service was deleted
 * @export
 * @interface WsMessageOneOf24
 */
export interface WsMessageOneOf24 {
    /**
     * The workspace this http service is related to
     * @type {string}
     * @memberof WsMessageOneOf24
     */
    workspace: string;
    /**
     * The uuid of the deleted http service
     * @type {string}
     * @memberof WsMessageOneOf24
     */
    httpService: string;
    /**
     * 
     * @type {string}
     * @memberof WsMessageOneOf24
     */
    type: WsMessageOneOf24TypeEnum;
}


/**
 * @export
 */
export const WsMessageOneOf24TypeEnum = {
    DeletedHttpService: 'DeletedHttpService'
} as const;
export type WsMessageOneOf24TypeEnum = typeof WsMessageOneOf24TypeEnum[keyof typeof WsMessageOneOf24TypeEnum];


/**
 * Check if a given object implements the WsMessageOneOf24 interface.
 */
export function instanceOfWsMessageOneOf24(value: object): boolean {
    let isInstance = true;
    isInstance = isInstance && "workspace" in value;
    isInstance = isInstance && "httpService" in value;
    isInstance = isInstance && "type" in value;

    return isInstance;
}

export function WsMessageOneOf24FromJSON(json: any): WsMessageOneOf24 {
    return WsMessageOneOf24FromJSONTyped(json, false);
}

export function WsMessageOneOf24FromJSONTyped(json: any, ignoreDiscriminator: boolean): WsMessageOneOf24 {
    if ((json === undefined) || (json === null)) {
        return json;
    }
    return {
        
        'workspace': json['workspace'],
        'httpService': json['http_service'],
        'type': json['type'],
    };
}

export function WsMessageOneOf24ToJSON(value?: WsMessageOneOf24 | null): any {
    if (value === undefined) {
        return undefined;
    }
    if (value === null) {
        return null;
    }
    return {
        
        'workspace': value.workspace,
        'http_service': value.httpService,
        'type': value.type,
    };
}

