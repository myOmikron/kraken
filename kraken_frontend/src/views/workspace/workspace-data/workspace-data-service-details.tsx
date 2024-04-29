import React, { useState } from "react";
import { toast } from "react-toastify";
import { Api } from "../../../api/api";
import { FullAggregationSource, FullService, ListFindings, ServiceRelations, TagType } from "../../../api/generated";
import SelectableText from "../../../components/selectable-text";
import Textarea from "../../../components/textarea";
import { handleApiError } from "../../../utils/helper";
import CertaintyIcon from "../components/certainty-icon";
import EditableTags from "../components/editable-tags";
import { ServiceRelationsList } from "../components/relations-list";
import SeverityIcon from "../components/severity-icon";
import { WORKSPACE_CONTEXT } from "../workspace";
import WorkspaceDataDetailsFindings from "./workspace-data-details-findings";
import WorkspaceDataDetailsResults from "./workspace-data-details-results";

/** React props for [`<WorkspaceDataServiceDetails />`]{@link WorkspaceDataServiceDetails} */
export type WorkspaceDataServiceDetailsProps = {
    /**
     * Service UUID
     */
    service: string;
    /**
     * Callback when Service data was edited
     */
    updateService?: (uuid: string, update: Partial<FullService>) => void;
    /**
     * The tab to render
     */
    tab: "general" | "results" | "relations" | "findings";
};

/**
 * Tall pane with background which shows detailed information for the Service
 */
export function WorkspaceDataServiceDetails(props: WorkspaceDataServiceDetailsProps) {
    const { service: uuid, updateService: signalUpdate, tab: tab } = props;
    const {
        workspace: { uuid: workspace },
    } = React.useContext(WORKSPACE_CONTEXT);
    const [attacks, setAttacks] = useState({} as FullAggregationSource);
    const [service, setService] = React.useState<FullService | null>(null);
    const [relations, setRelations] = React.useState<ServiceRelations | null>(null);
    const [findings, setFindings] = React.useState<ListFindings | null>(null);
    React.useEffect(() => {
        Api.workspaces.services.get(workspace, uuid).then(handleApiError(setService));
        Api.workspaces.services.relations(workspace, uuid).then(handleApiError(setRelations));
        Api.workspaces.services.findings(workspace, uuid).then(handleApiError(setFindings));
        Api.workspaces.services.sources(workspace, uuid).then(handleApiError(setAttacks));
    }, [workspace, uuid]);

    /**
     * Send an update to the server and parent component
     *
     * @param uuid  Service UUID to update
     * @param update The properties to update on the Service
     * @param msg Message to show on success
     */
    function update(uuid: string, update: Partial<FullService>, msg?: string) {
        const { tags, comment } = update;
        Api.workspaces.services
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

    if (service === null) return null;
    switch (tab) {
        case "general":
            return (
                <>
                    <div className={"workspace-data-details-pane"}>
                        <h3 className={"sub-heading"}>Service</h3>
                        {`${service.name} running on ${service.host.ipAddr}`}
                        {!service.port ? "" : ` (Port ${service.port.port}, ${service.port.protocol})`}
                    </div>
                    <div className="workspace-data-details-pane">
                        <h3 className="sub-heading">Certainty</h3>
                        <div className="workspace-data-certainty-list">
                            <CertaintyIcon certainty={service.certainty} />
                        </div>
                    </div>
                    {service.severity && (
                        <div className="workspace-data-details-pane">
                            <h3 className="sub-heading">Severity</h3>
                            <div className="workspace-data-certainty-list">
                                <SeverityIcon
                                    tooltip={false}
                                    className={"icon workspace-data-certainty-icon"}
                                    severity={service.severity}
                                />
                                {service.severity}
                            </div>
                        </div>
                    )}
                    <div className={"workspace-data-details-pane"}>
                        <h3 className={"sub-heading"}>Comment</h3>
                        <Textarea value={service.comment} onChange={(comment) => setService({ ...service, comment })} />
                        <button
                            className={"button"}
                            onClick={() =>
                                service && update(service.uuid, { comment: service.comment }, "Updated comment")
                            }
                        >
                            Update
                        </button>
                    </div>
                    <div className={"workspace-data-details-pane"}>
                        <h3 className={"sub-heading"}>Tags</h3>
                        <EditableTags
                            workspace={workspace}
                            tags={service.tags}
                            onChange={(tags) => {
                                setService((service) => service && { ...service, tags });
                                update(service.uuid, { tags });
                            }}
                        />
                    </div>
                    <SelectableText className="uuid">{uuid}</SelectableText>
                </>
            );
        case "results":
            return (
                <div className="workspace-data-details-flex">
                    <WorkspaceDataDetailsResults attacks={attacks.attacks} />
                </div>
            );
        case "relations":
            return (
                <div className="workspace-data-details-overflow">
                    <ServiceRelationsList relations={relations} />
                </div>
            );
        case "findings":
            return (
                <div className="workspace-data-details-overflow">
                    <WorkspaceDataDetailsFindings findings={findings} />
                </div>
            );
    }
}
