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
 * The request type to update a workspace
 * 
 * All parameter are optional, but at least one of them must be specified
 * @export
 * @interface UpdateWorkspaceRequest
 */
export interface UpdateWorkspaceRequest {
    /**
     * Name of the workspace
     * @type {string}
     * @memberof UpdateWorkspaceRequest
     */
    name?: string | null;
    /**
     * Description of the workspace
     * @type {string}
     * @memberof UpdateWorkspaceRequest
     */
    description?: string | null;
}

/**
 * Check if a given object implements the UpdateWorkspaceRequest interface.
 */
export function instanceOfUpdateWorkspaceRequest(value: object): boolean {
    let isInstance = true;

    return isInstance;
}

export function UpdateWorkspaceRequestFromJSON(json: any): UpdateWorkspaceRequest {
    return UpdateWorkspaceRequestFromJSONTyped(json, false);
}

export function UpdateWorkspaceRequestFromJSONTyped(json: any, ignoreDiscriminator: boolean): UpdateWorkspaceRequest {
    if ((json === undefined) || (json === null)) {
        return json;
    }
    return {
        
        'name': !exists(json, 'name') ? undefined : json['name'],
        'description': !exists(json, 'description') ? undefined : json['description'],
    };
}

export function UpdateWorkspaceRequestToJSON(value?: UpdateWorkspaceRequest | null): any {
    if (value === undefined) {
        return undefined;
    }
    if (value === null) {
        return null;
    }
    return {
        
        'name': value.name,
        'description': value.description,
    };
}

