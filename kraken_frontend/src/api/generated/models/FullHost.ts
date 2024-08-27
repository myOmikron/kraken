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
import type { FindingSeverity } from './FindingSeverity';
import {
    FindingSeverityFromJSON,
    FindingSeverityFromJSONTyped,
    FindingSeverityToJSON,
} from './FindingSeverity';
import type { HostCertainty } from './HostCertainty';
import {
    HostCertaintyFromJSON,
    HostCertaintyFromJSONTyped,
    HostCertaintyToJSON,
} from './HostCertainty';
import type { OsType } from './OsType';
import {
    OsTypeFromJSON,
    OsTypeFromJSONTyped,
    OsTypeToJSON,
} from './OsType';
import type { SimpleAggregationSource } from './SimpleAggregationSource';
import {
    SimpleAggregationSourceFromJSON,
    SimpleAggregationSourceFromJSONTyped,
    SimpleAggregationSourceToJSON,
} from './SimpleAggregationSource';
import type { SimpleTag } from './SimpleTag';
import {
    SimpleTagFromJSON,
    SimpleTagFromJSONTyped,
    SimpleTagToJSON,
} from './SimpleTag';

/**
 * The full representation of a host
 * @export
 * @interface FullHost
 */
export interface FullHost {
    /**
     * The primary key of the host
     * @type {string}
     * @memberof FullHost
     */
    uuid: string;
    /**
     * The ip address of the host
     * @type {string}
     * @memberof FullHost
     */
    ipAddr: string;
    /**
     * 
     * @type {OsType}
     * @memberof FullHost
     */
    osType: OsType;
    /**
     * Response time in ms
     * @type {number}
     * @memberof FullHost
     */
    responseTime?: number | null;
    /**
     * A comment
     * @type {string}
     * @memberof FullHost
     */
    comment: string;
    /**
     * The workspace this host is in
     * @type {string}
     * @memberof FullHost
     */
    workspace: string;
    /**
     * The list of tags this host has attached to
     * @type {Array<SimpleTag>}
     * @memberof FullHost
     */
    tags: Array<SimpleTag>;
    /**
     * 
     * @type {SimpleAggregationSource}
     * @memberof FullHost
     */
    sources: SimpleAggregationSource;
    /**
     * The point in time, the record was created
     * @type {Date}
     * @memberof FullHost
     */
    createdAt: Date;
    /**
     * 
     * @type {FindingSeverity}
     * @memberof FullHost
     */
    severity?: FindingSeverity | null;
    /**
     * 
     * @type {HostCertainty}
     * @memberof FullHost
     */
    certainty: HostCertainty;
}

/**
 * Check if a given object implements the FullHost interface.
 */
export function instanceOfFullHost(value: object): boolean {
    let isInstance = true;
    isInstance = isInstance && "uuid" in value;
    isInstance = isInstance && "ipAddr" in value;
    isInstance = isInstance && "osType" in value;
    isInstance = isInstance && "comment" in value;
    isInstance = isInstance && "workspace" in value;
    isInstance = isInstance && "tags" in value;
    isInstance = isInstance && "sources" in value;
    isInstance = isInstance && "createdAt" in value;
    isInstance = isInstance && "certainty" in value;

    return isInstance;
}

export function FullHostFromJSON(json: any): FullHost {
    return FullHostFromJSONTyped(json, false);
}

export function FullHostFromJSONTyped(json: any, ignoreDiscriminator: boolean): FullHost {
    if ((json === undefined) || (json === null)) {
        return json;
    }
    return {
        
        'uuid': json['uuid'],
        'ipAddr': json['ip_addr'],
        'osType': OsTypeFromJSON(json['os_type']),
        'responseTime': !exists(json, 'response_time') ? undefined : json['response_time'],
        'comment': json['comment'],
        'workspace': json['workspace'],
        'tags': ((json['tags'] as Array<any>).map(SimpleTagFromJSON)),
        'sources': SimpleAggregationSourceFromJSON(json['sources']),
        'createdAt': (new Date(json['created_at'])),
        'severity': !exists(json, 'severity') ? undefined : FindingSeverityFromJSON(json['severity']),
        'certainty': HostCertaintyFromJSON(json['certainty']),
    };
}

export function FullHostToJSON(value?: FullHost | null): any {
    if (value === undefined) {
        return undefined;
    }
    if (value === null) {
        return null;
    }
    return {
        
        'uuid': value.uuid,
        'ip_addr': value.ipAddr,
        'os_type': OsTypeToJSON(value.osType),
        'response_time': value.responseTime,
        'comment': value.comment,
        'workspace': value.workspace,
        'tags': ((value.tags as Array<any>).map(SimpleTagToJSON)),
        'sources': SimpleAggregationSourceToJSON(value.sources),
        'created_at': (value.createdAt.toISOString()),
        'severity': FindingSeverityToJSON(value.severity),
        'certainty': HostCertaintyToJSON(value.certainty),
    };
}

