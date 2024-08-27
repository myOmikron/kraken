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
 * 
 * @export
 * @interface GetAllHostsQueryAllOf
 */
export interface GetAllHostsQueryAllOf {
    /**
     * An optional general filter to apply
     * @type {string}
     * @memberof GetAllHostsQueryAllOf
     */
    globalFilter?: string | null;
    /**
     * An optional host specific filter to apply
     * @type {string}
     * @memberof GetAllHostsQueryAllOf
     */
    hostFilter?: string | null;
}

/**
 * Check if a given object implements the GetAllHostsQueryAllOf interface.
 */
export function instanceOfGetAllHostsQueryAllOf(value: object): boolean {
    let isInstance = true;

    return isInstance;
}

export function GetAllHostsQueryAllOfFromJSON(json: any): GetAllHostsQueryAllOf {
    return GetAllHostsQueryAllOfFromJSONTyped(json, false);
}

export function GetAllHostsQueryAllOfFromJSONTyped(json: any, ignoreDiscriminator: boolean): GetAllHostsQueryAllOf {
    if ((json === undefined) || (json === null)) {
        return json;
    }
    return {
        
        'globalFilter': !exists(json, 'global_filter') ? undefined : json['global_filter'],
        'hostFilter': !exists(json, 'host_filter') ? undefined : json['host_filter'],
    };
}

export function GetAllHostsQueryAllOfToJSON(value?: GetAllHostsQueryAllOf | null): any {
    if (value === undefined) {
        return undefined;
    }
    if (value === null) {
        return null;
    }
    return {
        
        'global_filter': value.globalFilter,
        'host_filter': value.hostFilter,
    };
}

