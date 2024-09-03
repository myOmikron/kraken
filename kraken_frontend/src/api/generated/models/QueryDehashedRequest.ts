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
import type { Query } from './Query';
import {
    QueryFromJSON,
    QueryFromJSONTyped,
    QueryToJSON,
} from './Query';

/**
 * The request to query the dehashed API
 * @export
 * @interface QueryDehashedRequest
 */
export interface QueryDehashedRequest {
    /**
     * 
     * @type {Query}
     * @memberof QueryDehashedRequest
     */
    query: Query;
    /**
     * The workspace to execute the attack in
     * @type {string}
     * @memberof QueryDehashedRequest
     */
    workspaceUuid: string;
}

/**
 * Check if a given object implements the QueryDehashedRequest interface.
 */
export function instanceOfQueryDehashedRequest(value: object): boolean {
    let isInstance = true;
    isInstance = isInstance && "query" in value;
    isInstance = isInstance && "workspaceUuid" in value;

    return isInstance;
}

export function QueryDehashedRequestFromJSON(json: any): QueryDehashedRequest {
    return QueryDehashedRequestFromJSONTyped(json, false);
}

export function QueryDehashedRequestFromJSONTyped(json: any, ignoreDiscriminator: boolean): QueryDehashedRequest {
    if ((json === undefined) || (json === null)) {
        return json;
    }
    return {
        
        'query': QueryFromJSON(json['query']),
        'workspaceUuid': json['workspace_uuid'],
    };
}

export function QueryDehashedRequestToJSON(value?: QueryDehashedRequest | null): any {
    if (value === undefined) {
        return undefined;
    }
    if (value === null) {
        return null;
    }
    return {
        
        'query': QueryToJSON(value.query),
        'workspace_uuid': value.workspaceUuid,
    };
}

