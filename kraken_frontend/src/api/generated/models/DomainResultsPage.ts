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
import type { SimpleDomain } from './SimpleDomain';
import {
    SimpleDomainFromJSON,
    SimpleDomainFromJSONTyped,
    SimpleDomainToJSON,
} from './SimpleDomain';

/**
 * Response containing paginated data
 * @export
 * @interface DomainResultsPage
 */
export interface DomainResultsPage {
    /**
     * The page's items
     * @type {Array<SimpleDomain>}
     * @memberof DomainResultsPage
     */
    items: Array<SimpleDomain>;
    /**
     * The limit this page was retrieved with
     * @type {number}
     * @memberof DomainResultsPage
     */
    limit: number;
    /**
     * The offset this page was retrieved with
     * @type {number}
     * @memberof DomainResultsPage
     */
    offset: number;
    /**
     * The total number of items this page is a subset of
     * @type {number}
     * @memberof DomainResultsPage
     */
    total: number;
}

/**
 * Check if a given object implements the DomainResultsPage interface.
 */
export function instanceOfDomainResultsPage(value: object): boolean {
    let isInstance = true;
    isInstance = isInstance && "items" in value;
    isInstance = isInstance && "limit" in value;
    isInstance = isInstance && "offset" in value;
    isInstance = isInstance && "total" in value;

    return isInstance;
}

export function DomainResultsPageFromJSON(json: any): DomainResultsPage {
    return DomainResultsPageFromJSONTyped(json, false);
}

export function DomainResultsPageFromJSONTyped(json: any, ignoreDiscriminator: boolean): DomainResultsPage {
    if ((json === undefined) || (json === null)) {
        return json;
    }
    return {
        
        'items': ((json['items'] as Array<any>).map(SimpleDomainFromJSON)),
        'limit': json['limit'],
        'offset': json['offset'],
        'total': json['total'],
    };
}

export function DomainResultsPageToJSON(value?: DomainResultsPage | null): any {
    if (value === undefined) {
        return undefined;
    }
    if (value === null) {
        return null;
    }
    return {
        
        'items': ((value.items as Array<any>).map(SimpleDomainToJSON)),
        'limit': value.limit,
        'offset': value.offset,
        'total': value.total,
    };
}
