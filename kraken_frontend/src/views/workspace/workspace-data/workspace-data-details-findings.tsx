import React from "react";
import Popup from "reactjs-popup";
import { FindingSeverity, ListFindings } from "../../../api/generated";
import FindingCategoryList from "../../../components/finding-category-list";
import { ROUTES } from "../../../routes";
import SeverityIcon from "../components/severity-icon";
import { WORKSPACE_CONTEXT } from "../workspace";

const SEVERITY_SORTING: { [k in FindingSeverity]: number } = {
    Okay: 0,
    Low: 1,
    Medium: 2,
    High: 3,
    Critical: 4,
};

/** React props for [`<WorkspaceDataDetailsFindings />`]{@link WorkspaceDataDetailsFindings}*/
type WorkspaceDataDetailsFindingsProps = {
    /**
     * interface which contains an Array of SimpleFindings
     */
    findings: ListFindings | null;
} & React.HTMLAttributes<HTMLDivElement>;

/**
 * simple list with a round border and blue background
 *
 * lists all findings given via props
 * displaying Severity, CVE and Name
 *
 * Click on one finding to redirect to the related finding page
 */
export default function WorkspaceDataDetailsFindings(props: WorkspaceDataDetailsFindingsProps) {
    const {
        workspace: { uuid: workspace },
    } = React.useContext(WORKSPACE_CONTEXT);
    const { findings, ...passThrough } = props;
    return (
        <div className="workspace-data-details-relations-container" {...passThrough}>
            <div className="workspace-data-details-relations-header workspace-data-details-findings">
                <div className="workspace-data-details-relations-heading">Severity</div>
                <div className="workspace-data-details-relations-heading">CVE</div>
                <div className="workspace-data-details-relations-heading" style={{ justifyContent: "start" }}>
                    Name
                </div>
            </div>
            {findings ? (
                <div className="workspace-data-details-relations-body workspace-data-details-findings">
                    {findings.findings
                        .sort((a, b) => {
                            return SEVERITY_SORTING[b.severity] - SEVERITY_SORTING[a.severity];
                        })
                        .map((r) => (
                            <div
                                className="workspace-data-details-relations-entry"
                                {...ROUTES.WORKSPACE_FINDINGS_EDIT.clickHandler({
                                    wUuid: workspace,
                                    fUuid: r.uuid,
                                })}
                            >
                                <SeverityIcon severity={r.severity} />
                                <span>{r.cve}</span>
                                <Popup
                                    position={"bottom center"}
                                    on={"hover"}
                                    arrow={true}
                                    trigger={<span>{r.name}</span>}
                                >
                                    <div className="pane-thin workspace-data-details-finding-popup">
                                        {r.categories.length === 0 ? (
                                            "No categories"
                                        ) : (
                                            <FindingCategoryList categories={r.categories} />
                                        )}
                                    </div>
                                </Popup>
                            </div>
                        ))}
                </div>
            ) : (
                <p>Loading...</p>
            )}
        </div>
    );
}
