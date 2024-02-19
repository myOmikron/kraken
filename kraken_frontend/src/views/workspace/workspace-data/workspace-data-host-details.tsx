import { Api } from "../../../api/api";
import React from "react";
import { FullAggregationSource, FullHost, HostRelations, TagType } from "../../../api/generated";
import { handleApiError } from "../../../utils/helper";
import Textarea from "../../../components/textarea";
import { toast } from "react-toastify";
import EditableTags from "../components/editable-tags";
import { WORKSPACE_CONTEXT } from "../workspace";
import "../../../styling/workspace-data-details.css";
import WorkspaceDataDetailsResults from "./workspace-data-details-results";
import ArrowLeftIcon from "../../../svg/arrow-left";
import ArrowRightIcon from "../../../svg/arrow-right";
import { useState } from "react";
import RelationRightIcon from "../../../svg/relation-right";
import RelationIndirectIcon from "../../../svg/relation-indirect";
import RelationLeftIcon from "../../../svg/relation-left";
import { CertaintyIcon } from "../workspace-data";

export type WorkspaceDataHostDetailsProps = {
    host: string;
    updateHost?: (uuid: string, update: Partial<FullHost>) => void;
    tab: "general" | "results" | "relations" | "findings";
};

export function WorkspaceDataHostDetails(props: WorkspaceDataHostDetailsProps) {
    const { host: uuid, updateHost: signalUpdate, tab: tab } = props;
    const {
        workspace: { uuid: workspace },
    } = React.useContext(WORKSPACE_CONTEXT);
    const [attacks, setAttacks] = useState({} as FullAggregationSource);
    const [limit, setLimit] = useState(0);
    const [page, setPage] = useState(0);
    const [host, setHost] = React.useState<FullHost | null>(null);
    const [relations, setRelations] = React.useState<HostRelations | null>(null);
    React.useEffect(() => {
        Api.workspaces.hosts.get(workspace, uuid).then(handleApiError(setHost));
        Api.workspaces.hosts.relations(workspace, uuid).then(handleApiError(setRelations));
        Api.workspaces.hosts.sources(workspace, uuid).then(
            handleApiError((x) => {
                setAttacks(x);
                setLimit(x.attacks.length - 1);
            }),
        );
    }, [workspace, uuid]);
    React.useEffect(() => {
        setPage(0);
    }, [uuid]);

    /** Send an update to the server and parent component */
    function update(uuid: string, update: Partial<FullHost>, msg?: string) {
        const { tags, comment } = update;
        Api.workspaces.hosts
            .update(workspace, uuid, {
                comment,
                workspaceTags:
                    tags && tags.filter(({ tagType }) => tagType === TagType.Workspace).map(({ uuid }) => uuid),
                globalTags: tags && tags.filter(({ tagType }) => tagType === TagType.Global).map(({ uuid }) => uuid),
            })
            .then(
                handleApiError(() => {
                    if (msg !== undefined) toast.success(msg);
                    if (signalUpdate !== undefined) signalUpdate(uuid, update);
                }),
            );
    }

    if (host === null) return null;
    return (
        <>
            {tab === "general" ? (
                <>
                    <div className="workspace-data-details-pane">
                        <h3 className={"sub-heading"}>Host</h3>
                        {host.ipAddr}
                    </div>
                    <div className="workspace-data-details-pane">
                        <h3 className="sub-heading">Certainty</h3>
                        <div className="workspace-data-certainty-list">
                            {host.certainty === "Verified"
                                ? CertaintyIcon({ certaintyType: "Verified", nameVisible: true })
                                : host.certainty === "Historical"
                                  ? CertaintyIcon({ certaintyType: "Historical", nameVisible: true })
                                  : CertaintyIcon({ certaintyType: "SupposedTo", nameVisible: true })}
                        </div>
                    </div>
                    <div className="workspace-data-details-pane">
                        <h3 className={"sub-heading"}>Comment</h3>
                        <Textarea value={host.comment} onChange={(comment) => setHost({ ...host, comment })} />
                        <button
                            className={"button"}
                            onClick={() => host && update(host.uuid, { comment: host.comment }, "Updated comment")}
                        >
                            Update
                        </button>
                    </div>
                    <div className="workspace-data-details-pane">
                        <h3 className={"sub-heading"}>Tags</h3>
                        <EditableTags
                            workspace={workspace}
                            tags={host.tags}
                            onChange={(tags) => {
                                setHost((host) => host && { ...host, tags });
                                update(host.uuid, { tags });
                            }}
                        />
                    </div>
                </>
            ) : (
                <>
                    {tab === "results" ? (
                        <div className="workspace-data-details-flex">
                            <WorkspaceDataDetailsResults attack={attacks.attacks[page]} uuid={host.uuid} />
                            <div className="workspace-data-details-table-controls">
                                <div className="workspace-data-details-controls-container">
                                    <button
                                        className={"workspace-table-button"}
                                        disabled={page === 0}
                                        onClick={() => {
                                            setPage(page - 1);
                                        }}
                                    >
                                        <ArrowLeftIcon />
                                    </button>
                                    <div className="workspace-table-controls-page-container">
                                        <span>
                                            {page + 1} of {limit + 1}
                                        </span>
                                    </div>
                                    <button
                                        className={"workspace-table-button"}
                                        disabled={page === limit}
                                        onClick={() => {
                                            setPage(page + 1);
                                        }}
                                    >
                                        <ArrowRightIcon />
                                    </button>
                                </div>
                            </div>
                        </div>
                    ) : (
                        <>
                            {tab === "relations" ? (
                                <div className="workspace-data-details-overflow">
                                    <div className="workspace-data-details-relations-container">
                                        <div className="workspace-data-details-relations-header">
                                            <div className="workspace-data-details-relations-heading">Connection</div>
                                            <div className="workspace-data-details-relations-heading">Type</div>
                                            <div className="workspace-data-details-relations-heading">To</div>
                                        </div>
                                        <div className="workspace-data-details-relations-body">
                                            {relations?.directDomains.map((d) => {
                                                return (
                                                    <div className="workspace-data-details-relations-entry">
                                                        <div title={"Direct"}>
                                                            <RelationLeftIcon />
                                                        </div>
                                                        <span>Domain</span>
                                                        <span>{d.domain} </span>
                                                    </div>
                                                );
                                            })}
                                            {relations?.indirectDomains.map((d) => {
                                                return (
                                                    <div className="workspace-data-details-relations-entry">
                                                        <div className="indirect" title={"Indirect"}>
                                                            <RelationIndirectIcon />
                                                        </div>
                                                        <span>Domain</span>
                                                        <span>{d.domain} </span>
                                                    </div>
                                                );
                                            })}
                                            {relations?.ports.map((p) => {
                                                return (
                                                    <div className="workspace-data-details-relations-entry">
                                                        <div title={"Direct"}>
                                                            <RelationRightIcon />
                                                        </div>
                                                        <span>Port</span>
                                                        <span>{p.port} </span>
                                                    </div>
                                                );
                                            })}
                                            {relations?.services.map((s) => {
                                                return (
                                                    <div className="workspace-data-details-relations-entry">
                                                        <div title={"Direct"}>
                                                            <RelationRightIcon />
                                                        </div>
                                                        <span>Service</span>
                                                        <span>{s.name} </span>
                                                    </div>
                                                );
                                            })}
                                        </div>
                                    </div>
                                </div>
                            ) : (
                                <>
                                    {tab === "findings" ? (
                                        <div className="workspace-data-details-overflow">
                                            <div className="workspace-data-details-relations-container">
                                                <div className="workspace-data-details-relations-header workspace-data-details-findings">
                                                    <div className="workspace-data-details-relations-heading">
                                                        Severity
                                                    </div>
                                                    <div className="workspace-data-details-relations-heading">CVE</div>
                                                    <div className="workspace-data-details-relations-heading">Name</div>
                                                </div>
                                                <div className="workspace-data-details-relations-body"></div>
                                            </div>
                                        </div>
                                    ) : (
                                        <div>Unimplemented</div>
                                    )}
                                </>
                            )}
                        </>
                    )}
                </>
            )}
        </>
    );
}
