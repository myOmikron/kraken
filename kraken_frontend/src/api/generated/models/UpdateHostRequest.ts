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
 * The request to update a host
 * @export
 * @interface UpdateHostRequest
 */
export interface UpdateHostRequest {
    /**
     * The comment of a host
     * @type {string}
     * @memberof UpdateHostRequest
     */
    comment?: string | null;
    /**
     * The global tags of a host
     * @type {Array<string>}
     * @memberof UpdateHostRequest
     */
    globalTags?: Array<string> | null;
    /**
     * The workspace tags of a host
     * @type {Array<string>}
     * @memberof UpdateHostRequest
     */
    workspaceTags?: Array<string> | null;
}

/**
 * Check if a given object implements the UpdateHostRequest interface.
 */
export function instanceOfUpdateHostRequest(value: object): boolean {
    let isInstance = true;

    return isInstance;
}

export function UpdateHostRequestFromJSON(json: any): UpdateHostRequest {
    return UpdateHostRequestFromJSONTyped(json, false);
}

export function UpdateHostRequestFromJSONTyped(json: any, ignoreDiscriminator: boolean): UpdateHostRequest {
    if ((json === undefined) || (json === null)) {
        return json;
    }
    return {
        
        'comment': !exists(json, 'comment') ? undefined : json['comment'],
        'globalTags': !exists(json, 'global_tags') ? undefined : json['global_tags'],
        'workspaceTags': !exists(json, 'workspace_tags') ? undefined : json['workspace_tags'],
    };
}

export function UpdateHostRequestToJSON(value?: UpdateHostRequest | null): any {
    if (value === undefined) {
        return undefined;
    }
    if (value === null) {
        return null;
    }
    return {
        
        'comment': value.comment,
        'global_tags': value.globalTags,
        'workspace_tags': value.workspaceTags,
    };
}

