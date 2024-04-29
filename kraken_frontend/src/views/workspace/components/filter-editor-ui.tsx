import { FC, useState } from "react";
import {
    FullDomain,
    FullHost,
    FullHttpService,
    FullPort,
    FullService,
    OsType,
    PortProtocol,
    SimpleTag,
} from "../../../api/generated";
import Textarea from "../../../components/textarea";
import "../../../styling/filter-editor-ui.css";
import CollapseIcon from "../../../svg/collapse";
import ExpandIcon from "../../../svg/expand";
import { ASTField, ASTFieldTypes, ASTFields, UsedASTTypes } from "../../../utils/filter/ast";
import { SpanlessToken, tokenize, tokensToString, valueToString } from "../../../utils/filter/lexer";
import { getExprs, replaceRaw } from "../../../utils/filter/mutate";
import { parseUserPort } from "../../../utils/ports";
import EditableDataList, { EditableDataListProps } from "./editable-data-list";
import EditableTags from "./editable-tags";

/**
 * React props for the <FilterEditorUi> component
 */
export type FilterEditorProps = {
    /** Workspace UUID, passed into the individual filter input components */
    workspace: string;
    /** The AST as defined in ast.ts for which to generate the UI for */
    ast: keyof typeof ASTFields;
    /** The full filter string, updated by onChange */
    filter: string;
    /** Callback when `filter` should be changed */
    onChange: (newValue: string) => void;
    /** Callback when `filter` should be changed and submitted as filter to the server */
    onApply: (newApplied: string) => void;
};

/**
 * A large form auto-generating input fields for each possible filter column for
 * filter strings and allowing editing through GUI components as well as through
 * the filter language syntax.
 */
export function FilterEditorUi(props: FilterEditorProps) {
    const filterComponents: { [K in UsedASTTypes]?: FC<FilterComponentProps> } = {
        tags: FilterTagsSelector,
        domain: FilterDomainSelector,
        host: FilterHostSelector,
        service: FilterServiceSelector,
        httpService: FilterHttpServiceSelector,
        "mayberange.date": FilterDateSelector,
        "mayberange.port": FilterRawPortSelector,
        ostype: FilterOsTypeSelector,
        protocol: FilterPortProtocolSelector,
        transport: FilterServiceTransportSelector,
    };

    const [showAdvanced, setShowAdvanced] = useState(false);
    const [resetCount, setResetCount] = useState(0);

    const anyAdvanced = Object.entries(ASTFieldTypes[props.ast]).some(([key]) => {
        const ast = ASTFields[props.ast] as ASTField;
        const astField = ast[key];
        return astField.advanced;
    });

    return (
        <form
            className="filter-editor"
            onReset={() => {
                setResetCount((c) => c + 1);
                props.onChange("");
                props.onApply("");
            }}
            onSubmit={() => {}}
        >
            <main>
                {Object.entries(ASTFieldTypes[props.ast]).map(([key, type]) => {
                    const Component = filterComponents[type];
                    if (!Component) return undefined;
                    const ast = ASTFields[props.ast] as ASTField;
                    const astField = ast[key];
                    if (astField.advanced) return;
                    return <Component key={key} field={key} {...props} />;
                })}
            </main>
            {anyAdvanced && (
                <fieldset key={resetCount} className={`${showAdvanced ? "" : "collapsed"}`}>
                    <legend
                        onMouseDown={(e) => {
                            setShowAdvanced(!showAdvanced);
                            e.preventDefault();
                        }}
                    >
                        {showAdvanced ? <CollapseIcon /> : <ExpandIcon />} Advanced
                    </legend>
                    {showAdvanced &&
                        Object.entries(ASTFieldTypes[props.ast]).map(([key, type]) => {
                            const Component = filterComponents[type];
                            if (!Component) return undefined;
                            const ast = ASTFields[props.ast] as ASTField;
                            const astField = ast[key];
                            if (!astField.advanced) return;
                            return <Component key={key} field={key} {...props} />;
                        })}
                </fieldset>
            )}
            <footer>
                <Textarea
                    rows={2}
                    className={"input"}
                    placeholder={"Filter..."}
                    value={props.filter}
                    onChange={(v) => props.onChange(v.replace("\n", " "))}
                />
                <button className="button" type="reset">
                    Clear
                </button>
                <button className="button" type="submit">
                    Apply
                </button>
            </footer>
        </form>
    );
}

/**
 * Props for all the filter input component, which is used for each individual filter input.
 */
export type FilterComponentProps = {
    /**
     * The workspace UUID to fetch data in
     */
    workspace: string;
    /**
     * the AST key in ASTFields from which to load the filter AST from for UI and validation.
     */
    ast: keyof typeof ASTFields;
    /**
     * The field to edit
     */
    field: string; // keyof typeof ASTFields[ast]
    /**
     * The full filter string
     */
    filter: string;
    /**
     * Called when the component wants the filter to update.
     */
    onChange: (newValue: string) => void;
};

/**
 * Component to select a list of tags to filter.
 */
export function FilterTagsSelector(props: FilterComponentProps) {
    const ast = ASTFields[props.ast] as ASTField;
    const astField = ast[props.field];
    if (!astField) return undefined;

    const [allTags, setAllTags] = useState<SimpleTag[]>([]);

    // eslint-disable-next-line jsdoc/require-param, jsdoc/require-returns
    /** Resolves a tag label to a SimpleTag object. */
    function findTag(label: string): SimpleTag | undefined {
        return allTags.find((v) => v.name == label);
    }

    const values = astField.columns
        .map<[string, SpanlessToken[]]>((c) => [c, getExprs(props.filter, c)!])
        .find((c) => c[1] !== undefined);
    const usedColumn = values?.[0] ?? astField.columns[0];
    const tags =
        (values?.[1]
            .map((v) => (v.type == "value" ? findTag(v.value) : undefined))
            .filter((v) => v !== undefined) as SimpleTag[]) ?? [];

    return (
        <label>
            <span>{astField.label}</span>
            <EditableTags
                workspace={props.workspace}
                onTagsLoaded={setAllTags}
                tags={tags}
                allowCreate={false}
                onChange={(newTags: SimpleTag[]) => {
                    props.onChange(
                        replaceRaw(props.filter, usedColumn, newTags.map((t) => valueToString(t.name)).join(" & ")),
                    );
                }}
            />
        </label>
    );
}

/**
 * Component to select one of all available domains.
 */
export function FilterDomainSelector(props: FilterComponentProps) {
    return FilterDataSelector<FullDomain>({
        ...props,
        type: "domains",
        // eslint-disable-next-line jsdoc/require-jsdoc
        mapper: (v) => v.domain,
    });
}

/**
 * Component to select one of all available hosts.
 */
export function FilterHostSelector(props: FilterComponentProps) {
    return FilterDataSelector<FullHost>({
        ...props,
        type: "hosts",
        // eslint-disable-next-line jsdoc/require-jsdoc
        mapper: (v) => v.ipAddr,
    });
}

/**
 * Component to select one of all available services.
 */
export function FilterServiceSelector(props: FilterComponentProps) {
    return FilterDataSelector<FullService>({
        ...props,
        type: "services",
        // eslint-disable-next-line jsdoc/require-jsdoc
        mapper: (v) => v.name,
    });
}

/**
 * Component to select one of all available HTTP services.
 */
export function FilterHttpServiceSelector(props: FilterComponentProps) {
    return FilterDataSelector<FullHttpService>({
        ...props,
        type: "httpServices",
        // eslint-disable-next-line jsdoc/require-jsdoc
        mapper: (v) => v.name,
    });
}

/**
 * Component to select a data objects, automatically fetching the available list
 * of data from the API.
 */
function FilterDataSelector<T extends FullHost | FullPort | FullDomain | FullService | FullHttpService>(
    props: FilterComponentProps & {
        /**
         * the runtime type, matching `T`, for runtime type-dependent code.
         */
        type: EditableDataListProps<T>["type"];
        /**
         * Maps from a rich data object to what should be written & parsed
         * inside the filter code string.
         */
        mapper: (item: T) => string;
    },
) {
    const ast = ASTFields[props.ast] as ASTField;
    const astField = ast[props.field];
    if (!astField) return undefined;

    const [allDatas, setAllDatas] = useState<T[]>([]);

    // eslint-disable-next-line jsdoc/require-param, jsdoc/require-returns
    /** Resolves the data text as written by the user to a data object. */
    function findData(data: string): T | undefined {
        return allDatas.find((v) => props.mapper(v) == data);
    }

    const values = astField.columns
        .map<[string, SpanlessToken[]]>((c) => [c, getExprs(props.filter, c)!])
        .find((c) => c[1] !== undefined);
    const usedColumn = values?.[0] ?? astField.columns[0];
    const datas =
        (values?.[1]
            .map((v) => (v.type == "value" ? findData(v.value) : undefined))
            .filter((v) => v !== undefined) as T[]) ?? [];

    return (
        <label>
            <span>{astField.label}</span>
            <EditableDataList<T>
                workspace={props.workspace}
                onItemsLoaded={setAllDatas}
                type={props.type}
                items={datas}
                onChange={(newTags: T[]) => {
                    props.onChange(
                        replaceRaw(
                            props.filter,
                            usedColumn,
                            newTags.map((t) => valueToString(props.mapper(t))).join(", "),
                        ),
                    );
                }}
            />
        </label>
    );
}

/**
 * Component to select a date or a date range.
 */
function FilterDateSelector(props: FilterComponentProps) {
    const ast = ASTFields[props.ast] as ASTField;
    const astField = ast[props.field];
    if (!astField) return undefined;

    const values = astField.columns
        .map<[string, SpanlessToken[]]>((c) => [c, getExprs(props.filter, c)!])
        .find((c) => c[1] !== undefined);
    const usedColumn = values?.[0] ?? astField.columns[0];
    const tokens = values?.[1];
    const rangeOp = tokens?.findIndex((t) => t.type == "rangeOperator") ?? -1;
    const beforeTok = tokens?.[rangeOp - 1];
    const afterTok = tokens?.[rangeOp + 1];

    const minDateInput = beforeTok?.type == "value" ? new Date(beforeTok.value) : undefined;
    const maxDateInput = afterTok?.type == "value" ? new Date(afterTok.value) : undefined;

    const [minOverride, setMinOverride] = useState<Date | undefined>(undefined);
    const [maxOverride, setMaxOverride] = useState<Date | undefined>(undefined);

    const minDate = minOverride ?? minDateInput;
    const maxDate = maxOverride ?? maxDateInput;

    // eslint-disable-next-line jsdoc/require-param, jsdoc/require-returns
    /** returns a string suitable for input type="datetime-local" */
    function dateToString(d: Date): string {
        // eslint-disable-next-line jsdoc/require-param, jsdoc/require-returns
        /** single digit number to two digit numbers (0-9 to 00-09) */
        const pad2 = (n: number) => (n < 10 ? "0" + n.toFixed(0) : n.toFixed(0));
        return `${d.getFullYear()}-${pad2(d.getMonth() + 1)}-${pad2(d.getDate())}T${pad2(d.getHours())}:${pad2(d.getMinutes())}`;
    }

    // eslint-disable-next-line jsdoc/require-param, jsdoc/require-returns
    /**
     * Called to set the actual values. Undefined means unbounded. Only called
     * once validation succeeds so that the user doesn't have a poor editing
     * experience while entering partial dates.
     */
    function setDates(min: Date | undefined, max: Date | undefined) {
        props.onChange(
            replaceRaw(
                props.filter,
                usedColumn,
                min || max
                    ? (min ? valueToString(min.toISOString()) : "") +
                          "-" +
                          (max ? valueToString(max.toISOString()) : "")
                    : "",
            ),
        );
        setMaxOverride(undefined);
        setMinOverride(undefined);
    }

    return (
        <label className="label3">
            <span>{astField.label}</span>
            <input
                type="datetime-local"
                className="input"
                value={minDate ? dateToString(minDate) : undefined}
                max={maxDate ? dateToString(maxDate) : undefined}
                onChange={(e) => {
                    if (e.target.value) {
                        const date = new Date(e.target.value);
                        if (maxDate && date > maxDate) setMinOverride(date);
                        else setDates(date, maxDate);
                    } else {
                        setDates(undefined, maxDate);
                    }
                }}
            />
            <input
                type="datetime-local"
                className="input"
                value={maxDate ? dateToString(maxDate) : undefined}
                min={minDate ? dateToString(minDate) : undefined}
                onChange={(e) => {
                    if (e.target.value) {
                        const date = new Date(e.target.value);
                        if (minDate && date < minDate) setMaxOverride(date);
                        else setDates(minDate, date);
                    } else {
                        setDates(minDate, undefined);
                    }
                }}
            />
        </label>
    );
}

/**
 * Component to select a port number or range.
 */
function FilterRawPortSelector(props: FilterComponentProps) {
    const ast = ASTFields[props.ast] as ASTField;
    const astField = ast[props.field];
    if (!astField) return undefined;

    const values = astField.columns
        .map<[string, SpanlessToken[]]>((c) => [c, getExprs(props.filter, c)!])
        .find((c) => c[1] !== undefined);
    const usedColumn = values?.[0] ?? astField.columns[0];
    const tokens = values?.[1];
    const inputValue = tokens ? tokensToString(tokens) : undefined;

    const [override, setOverride] = useState<string | undefined>(undefined);

    const value = override ?? inputValue;

    // eslint-disable-next-line jsdoc/require-param, jsdoc/require-returns
    /** attempts to parse the port or throws on error */
    function validatePort(v: string): void {
        if (parseUserPort(v) === false) throw new Error("invalid port: " + v);
    }

    // eslint-disable-next-line jsdoc/require-param, jsdoc/require-returns
    /** attempts to parse all port tokens or throws on error */
    function validatePorts(tokens: SpanlessToken[]): void {
        tokens.forEach((t) => t.type == "value" && validatePort(t.value));
    }

    return (
        <label>
            <span>{astField.label}</span>
            <input
                type="text"
                className="input"
                value={value}
                onChange={(e) => {
                    if (e.target.value) {
                        try {
                            const tokens = tokenize(e.target.value);
                            validatePorts(tokens);
                            props.onChange(replaceRaw(props.filter, usedColumn, tokensToString(tokens)));
                        } catch (err) {
                            setOverride(e.target.value);
                            e.target.setCustomValidity("Invalid input: " + err);
                            return;
                        }
                    } else {
                        props.onChange(replaceRaw(props.filter, usedColumn, ""));
                    }
                    setOverride(undefined);
                    e.target.setCustomValidity("");
                }}
            />
        </label>
    );
}

/**
 * Component to select from the OsType enum using checkboxes.
 */
function FilterOsTypeSelector(props: FilterComponentProps) {
    return FilterCheckboxEnumSelector({
        ...props,
        enum: Object.values(OsType),
    });
}

/**
 * Component to select from the PortProtocol enum using checkboxes.
 */
function FilterPortProtocolSelector(props: FilterComponentProps) {
    return FilterCheckboxEnumSelector({
        ...props,
        enum: Object.values(PortProtocol),
    });
}

/**
 * Component to select "Raw" or "TLS" using checkboxes.
 */
function FilterServiceTransportSelector(props: FilterComponentProps) {
    return FilterCheckboxEnumSelector({
        ...props,
        enum: ["Raw", "TLS"],
    });
}

/**
 * Component to select from a list of all possible values using checkboxes.
 */
function FilterCheckboxEnumSelector(
    props: FilterComponentProps & {
        /**
         * The list of possible values as entered by the user.
         */
        enum: string[];
    },
) {
    const ast = ASTFields[props.ast] as ASTField;
    const astField = ast[props.field];
    if (!astField) return undefined;

    const values = astField.columns
        .map<[string, SpanlessToken[]]>((c) => [c, getExprs(props.filter, c)!])
        .find((c) => c[1] !== undefined);
    const usedColumn = values?.[0] ?? astField.columns[0];
    const tokens = values?.[1];

    const checked = tokens?.filter((v) => v.type == "value").map((v) => ("value" in v ? v.value : "")) ?? [];

    return (
        <div className="row checkbox-list">
            <span>{astField.label}</span>
            <div>
                {props.enum.map((enumValue) => (
                    <label key={enumValue}>
                        <input
                            type="checkbox"
                            className="input"
                            checked={checked.includes(enumValue)}
                            onChange={(e) => {
                                const newChecked = checked;
                                const existing = newChecked.indexOf(enumValue);
                                if (e.target.checked) {
                                    if (existing != -1) return;
                                    newChecked.push(enumValue);
                                } else {
                                    if (existing == -1) return;
                                    newChecked.splice(existing, 1);
                                }
                                props.onChange(
                                    replaceRaw(props.filter, usedColumn, newChecked.map(valueToString).join(", ")),
                                );
                            }}
                        />
                        {enumValue}
                    </label>
                ))}
            </div>
        </div>
    );
}
