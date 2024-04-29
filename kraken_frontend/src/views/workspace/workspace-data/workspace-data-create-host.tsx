import React from "react";
import Select from "react-select";
import { toast } from "react-toastify";
import { Api } from "../../../api/api";
import { ManualHostCertainty } from "../../../api/generated";
import Input from "../../../components/input";
import { selectStyles } from "../../../components/select-menu";
import { handleApiError } from "../../../utils/helper";
import { WORKSPACE_CONTEXT } from "../workspace";

/** React props for [`<CreateHostForm />`]{@link CreateHostForm} */
type CreateHostFormProps = {
    /**
     * Callback when new Host was successfully created
     */
    onSubmit: () => void;
};

/**
 * `<form />`including all inputs to manually create a new Host
 *
 * Allows submitting to create in the current workspace.
 */
export function CreateHostForm(props: CreateHostFormProps) {
    const { onSubmit } = props;
    const {
        workspace: { uuid: workspace },
    } = React.useContext(WORKSPACE_CONTEXT);
    const [ip, setIp] = React.useState("");
    const [certy, setCerty] = React.useState<ManualHostCertainty>("SupposedTo");
    return (
        <form
            className={"pane workspace-data-create-form"}
            onSubmit={(event) => {
                event.preventDefault();
                Api.workspaces.hosts.create(workspace, { ipAddr: ip, certainty: certy }).then(
                    handleApiError(() => {
                        toast.success("Added host");
                        onSubmit();
                    }),
                );
            }}
        >
            <h2 className={"sub-heading"}>Manually add a host</h2>
            <label>
                IP / net in CIDR:
                <Input value={ip} onChange={setIp} required />
            </label>
            <label>
                Certainty:
                <Select<{
                    /**
                     * selectable option for host certainty
                     */
                    value: ManualHostCertainty;
                    /**
                     * react select label for host certainty
                     */
                    label: ManualHostCertainty;
                }>
                    styles={selectStyles("default")}
                    options={Object.values(ManualHostCertainty).map((value) => ({ value, label: value }))}
                    value={{ value: certy, label: certy }}
                    onChange={(newValue) => setCerty(newValue?.value || certy)}
                />
            </label>
            <button className={"button"} type={"submit"}>
                Add
            </button>
        </form>
    );
}
