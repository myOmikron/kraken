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
import type { SimpleHost } from './SimpleHost';
import {
    SimpleHostFromJSON,
    SimpleHostFromJSONTyped,
    SimpleHostToJSON,
} from './SimpleHost';
import type { SimplePort } from './SimplePort';
import {
    SimplePortFromJSON,
    SimplePortFromJSONTyped,
    SimplePortToJSON,
} from './SimplePort';

/**
 * A service's direct relations
 * @export
 * @interface ServiceRelations
 */
export interface ServiceRelations {
    /**
     * 
     * @type {SimplePort}
     * @memberof ServiceRelations
     */
    port?: SimplePort | null;
    /**
     * 
     * @type {SimpleHost}
     * @memberof ServiceRelations
     */
    host: SimpleHost;
}

/**
 * Check if a given object implements the ServiceRelations interface.
 */
export function instanceOfServiceRelations(value: object): boolean {
    let isInstance = true;
    isInstance = isInstance && "host" in value;

    return isInstance;
}

export function ServiceRelationsFromJSON(json: any): ServiceRelations {
    return ServiceRelationsFromJSONTyped(json, false);
}

export function ServiceRelationsFromJSONTyped(json: any, ignoreDiscriminator: boolean): ServiceRelations {
    if ((json === undefined) || (json === null)) {
        return json;
    }
    return {
        
        'port': !exists(json, 'port') ? undefined : SimplePortFromJSON(json['port']),
        'host': SimpleHostFromJSON(json['host']),
    };
}

export function ServiceRelationsToJSON(value?: ServiceRelations | null): any {
    if (value === undefined) {
        return undefined;
    }
    if (value === null) {
        return null;
    }
    return {
        
        'port': SimplePortToJSON(value.port),
        'host': SimpleHostToJSON(value.host),
    };
}

