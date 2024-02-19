import React, { Context } from "react";
import { FullWorkspace, FullWorkspaceInvitation, SimpleUser, UserPermission } from "../../api/generated";
import "../../styling/workspace-settings.css";
import Input from "../../components/input";
import { Api } from "../../api/api";
import Textarea from "../../components/textarea";
import { toast } from "react-toastify";
import Tag from "../../components/tag";
import CloseIcon from "../../svg/close";
import Popup from "reactjs-popup";
import { ROUTES } from "../../routes";
import SelectMenu from "../../components/select-menu";
import Bubble from "../../components/bubble";
import Workspace, { WORKSPACE_CONTEXT, WorkspaceContext } from "./workspace";
import { handleApiError } from "../../utils/helper";
import USER_CONTEXT from "../../context/user";

type WorkspaceSettingsProps = {};
type WorkspaceSettingsState = {
    workspaceName: string;
    workspaceDescription: string | null;
    invitePopup: boolean;
    deleteUserPopup: boolean;
    selected: boolean;
    deleteWorkspacePopup: boolean;
    transferOwnershipPopup: boolean;
    memberName: string;
    transferList: Array<SelectValue>;
    inviteList: Array<SelectValue>;
    selectedUser: null | SelectValue;
    invitedUsers: Array<FullWorkspaceInvitation>;
};

type SelectValue = {
    label: string;
    value: string;
};

export default class WorkspaceSettings extends React.Component<WorkspaceSettingsProps, WorkspaceSettingsState> {
    static contextType = WORKSPACE_CONTEXT;
    declare context: React.ContextType<typeof WORKSPACE_CONTEXT>;

    state: WorkspaceSettingsState = {
        workspaceName: "",
        workspaceDescription: "",
        invitePopup: false,
        deleteUserPopup: false,
        deleteWorkspacePopup: false,
        transferOwnershipPopup: false,
        selected: false,
        memberName: "",
        transferList: [],
        inviteList: [],
        selectedUser: null,
        invitedUsers: [],
    };

    componentDidMount() {
        this.createTransferList().then();
        this.createInviteList().then();
        this.updateInvitedUsers().then();

        this.setState({
            workspaceName: this.context.workspace.name,
            workspaceDescription: this.context.workspace.description || null,
        });
    }

    async updateInvitedUsers() {
        await Api.workspaces.invitations
            .all(this.context.workspace.uuid)
            .then(handleApiError((x) => this.setState({ invitedUsers: x.invitations })));
    }

    async updateWorkspace() {
        let update: { name: null | string; description: null | string } = { name: null, description: null };

        if (this.state.workspaceName !== this.context.workspace.name && this.state.workspaceName !== "") {
            update = { ...update, name: this.state.workspaceName };
        }

        if (this.state.workspaceDescription !== this.context.workspace.description) {
            update = { ...update, description: this.state.workspaceDescription };
        }

        await Api.workspaces
            .update(this.context.workspace.uuid, update)
            .then(handleApiError(() => toast.success("Workspace updated")));
    }

    async deleteWorkspace() {
        const toastId = toast.loading("Deleting workspace");
        await Api.workspaces.delete(this.context.workspace.uuid).then(
            handleApiError(() => {
                toast.success("Deleted Workspace");
                ROUTES.WORKSPACES.visit({});
            }),
        );
        toast.dismiss(toastId);
    }

    async createTransferList() {
        await Api.user.all().then(
            handleApiError((u) => {
                u.users
                    .filter((s) => {
                        return this.context.workspace.owner.uuid !== s.uuid;
                    })
                    .map((s) => {
                        let member = { label: s.displayName + " (" + s.username + ") ", value: s.uuid };
                        this.state.transferList.push(member);
                    });
            }),
        );
    }

    async createInviteList() {
        await Api.user.all().then(
            handleApiError((u) => {
                u.users
                    .filter((s) => {
                        let users = [
                            ...this.context.workspace.members.map((x) => x.uuid),
                            this.context.workspace.owner.uuid,
                        ];
                        return !users.some((x) => x === s.uuid);
                    })
                    .map((s) => {
                        let member = { label: s.displayName + " (" + s.username + ") ", value: s.uuid };
                        this.state.inviteList.push(member);
                    });
            }),
        );
    }

    async deleteUser() {
        /*TODO delete member*/
    }

    async inviteUser() {
        if (this.state.selectedUser === null) {
            toast.error("No user selected");
            return;
        }

        await Api.workspaces.invitations.create(this.context.workspace.uuid, this.state.selectedUser.value).then(
            handleApiError(async () => {
                toast.success("Invitation was sent");
                this.setState({ selectedUser: null, invitePopup: false });
                await this.updateInvitedUsers();
            }),
        );
    }

    async transferOwnership() {
        if (this.state.selectedUser === null) {
            toast.error("No user selected");
            return;
        }
        await Api.workspaces.transferOwnership(this.context.workspace.uuid, this.state.selectedUser.value).then(
            handleApiError(() => {
                toast.success("Transfer was successful");
                this.setState({ selectedUser: null, transferOwnershipPopup: false, selected: false });
            }),
        );
    }

    render() {
        return (
            <>
                <USER_CONTEXT.Consumer>
                    {(ctx) => {
                        if (this.context.workspace.owner.uuid !== ctx.user.uuid) {
                            ROUTES.WORKSPACE_HOSTS.visit({ uuid: this.context.workspace.uuid });
                        }
                        return null;
                    }}
                </USER_CONTEXT.Consumer>
                <div className={"workspace-settings-layout"}>
                    <div className="workspace-settings-row">
                        <form
                            className="pane workspace-settings-container"
                            method={"post"}
                            onSubmit={async (x) => {
                                x.preventDefault();
                                await this.updateWorkspace();
                            }}
                        >
                            <h2 className={"sub-heading"}> Workspace Settings </h2>
                            <div className={"workspace-settings-table"}>
                                <span>Name</span>
                                <Input
                                    value={this.state.workspaceName}
                                    onChange={(v) => {
                                        this.setState({ workspaceName: v });
                                    }}
                                    placeholder={this.context.workspace.name}
                                />
                                <span>Description</span>
                                <Textarea
                                    value={
                                        this.state.workspaceDescription !== null &&
                                        this.state.workspaceDescription !== undefined
                                            ? this.state.workspaceDescription
                                            : ""
                                    }
                                    onChange={(v) => {
                                        this.setState({ workspaceDescription: v });
                                    }}
                                    placeholder={"Description"}
                                />
                            </div>
                            <button className={"button"}>Save</button>
                        </form>
                        <div className="workspace-settings-container danger pane">
                            <h2 className={"sub-heading"}>Danger Zone </h2>
                            <div className="workspace-settings-danger">
                                <span>Transfer ownership</span>
                                <button
                                    className="workspace-settings-red-button button"
                                    onClick={() => {
                                        this.setState({ transferOwnershipPopup: true });
                                    }}
                                >
                                    Transfer
                                </button>
                                <span>Delete this workspace</span>
                                <button
                                    className="workspace-settings-red-button button"
                                    onClick={() => {
                                        this.setState({ deleteWorkspacePopup: true });
                                    }}
                                >
                                    Delete
                                </button>
                            </div>
                        </div>
                    </div>
                    <div className="pane workspace-settings-container">
                        <h2 className={"sub-heading"}>User control</h2>
                        <div className={"workspace-settings-container"}>
                            <div className={"workspace-settings-invite"}>
                                <button
                                    className={"workspace-settings-button button"}
                                    onClick={() => {
                                        this.setState({ invitePopup: true });
                                    }}
                                >
                                    Invite
                                </button>
                            </div>
                            <div className={"workspace-settings-user-table-heading neon"}>
                                <span>Username</span>
                                <span>Role</span>
                                <span>Delete</span>
                            </div>
                            <div className={"workspace-settings-user-table-entry neon"}>
                                <span>{this.context.workspace.owner.displayName}</span>
                                <div className={"workspace-settings-tag-container"}>
                                    <Bubble name={"owner"} color={"primary"} />
                                </div>
                            </div>
                            {this.context.workspace.members.map((m) => (
                                <div key={m.uuid} className={"workspace-settings-user-table-entry neon"}>
                                    <span>{m.displayName}</span>
                                    <div className={"workspace-settings-tag-container"}>
                                        <Tag name={"member"} />
                                    </div>
                                    <span>
                                        <button
                                            className={"icon-button"}
                                            onClick={() => {
                                                this.setState({
                                                    deleteUserPopup: true,
                                                    memberName: m.displayName,
                                                });
                                            }}
                                        >
                                            <CloseIcon />
                                        </button>
                                    </span>
                                </div>
                            ))}
                            {this.state.invitedUsers.map((i) => (
                                <div key={i.uuid} className={"workspace-settings-user-table-entry neon"}>
                                    <span>{i.target.displayName}</span>
                                    <div className={"workspace-settings-tag-container"}>
                                        <Tag name={"invited"} />
                                    </div>
                                    <span>
                                        <button
                                            className={"icon-button"}
                                            onClick={async () => {
                                                await Api.workspaces.invitations
                                                    .retract(this.context.workspace.uuid, i.uuid)
                                                    .then(
                                                        handleApiError(async () => {
                                                            toast.success("Invitation retracted");
                                                            await this.updateInvitedUsers();
                                                        }),
                                                    );
                                            }}
                                        >
                                            <CloseIcon />
                                        </button>
                                    </span>
                                </div>
                            ))}
                        </div>
                    </div>
                    <div className="pane workspace-settings-container">
                        <h2 className={"sub-heading"}>Linked OAuth applications </h2>
                        <div className={"workspace-settings-oauth-table-heading neon"}>
                            <span>Date</span>
                            <span>Name</span>
                            <span>User</span>
                        </div>
                        {/*TODO show linked apps*/}
                    </div>
                </div>
                <Popup
                    modal={true}
                    nested={true}
                    open={this.state.invitePopup}
                    onClose={() => {
                        this.setState({ invitePopup: false, selectedUser: null });
                    }}
                >
                    <form
                        method={"post"}
                        className="workspace-settings-popup pane"
                        onSubmit={async (e) => {
                            e.preventDefault();
                            await this.inviteUser();
                        }}
                    >
                        <div className="workspace-setting-popup">
                            <h2 className="sub-heading"> Invite member</h2>
                            <SelectMenu
                                options={this.state.inviteList}
                                theme={"default"}
                                value={this.state.selectedUser}
                                onChange={(type) => {
                                    this.setState({ selectedUser: type });
                                }}
                            />
                            <button className="button">Invite</button>
                        </div>
                    </form>
                </Popup>
                <Popup
                    modal={true}
                    nested={true}
                    open={this.state.deleteUserPopup}
                    onClose={() => {
                        this.setState({ deleteUserPopup: false, memberName: "" });
                    }}
                >
                    <div className="popup-content pane">
                        <div className="workspace-setting-popup">
                            <h2 className="sub-heading">Delete {this.state.memberName} from this workspace?</h2>
                            <button
                                className="button"
                                onClick={() => {
                                    this.setState({ deleteUserPopup: false, memberName: "" });
                                }}
                            >
                                No
                            </button>
                            <button
                                className="button"
                                onClick={async (x) => {
                                    {
                                        x.preventDefault();
                                        await this.deleteUser();
                                    }
                                }}
                            >
                                Yes
                            </button>
                        </div>
                    </div>
                </Popup>
                <Popup
                    modal={true}
                    nested={true}
                    open={this.state.deleteWorkspacePopup}
                    onClose={() => {
                        this.setState({ deleteWorkspacePopup: false });
                    }}
                >
                    <div className="popup-content pane danger">
                        <div className="workspace-setting-popup">
                            <h2 className="sub-heading"> Are you sure to delete this workspace?</h2>
                            <span>All data will be lost upon deletion!</span>
                            <button
                                className="workspace-settings-red-button button"
                                onClick={() => {
                                    this.setState({ deleteWorkspacePopup: false });
                                }}
                            >
                                No
                            </button>
                            <button
                                className="workspace-settings-red-button button"
                                onClick={async (x) => {
                                    {
                                        x.preventDefault();
                                        await this.deleteWorkspace();
                                    }
                                }}
                            >
                                Yes
                            </button>
                        </div>
                    </div>
                </Popup>
                <Popup
                    modal={true}
                    nested={true}
                    open={this.state.transferOwnershipPopup}
                    onClose={() => {
                        this.setState({ transferOwnershipPopup: false, selectedUser: null, selected: false });
                    }}
                >
                    {this.state.selected && this.state.selectedUser !== null ? (
                        <form
                            className="workspace-setting-popup danger pane"
                            method={"post"}
                            onSubmit={async (e) => {
                                e.preventDefault();
                                await this.transferOwnership();
                                ROUTES.WORKSPACES.visit({});
                            }}
                        >
                            <h2 className="sub-heading">Transfer the ownership to {this.state.selectedUser?.label}?</h2>
                            <span> You will loose access to this workspace!</span>
                            <button className="workspace-settings-red-button button">Transfer</button>
                            <button
                                className="workspace-settings-red-button button"
                                onClick={() => {
                                    this.setState({
                                        transferOwnershipPopup: false,
                                        selectedUser: null,
                                        selected: false,
                                    });
                                }}
                            >
                                Abort
                            </button>
                        </form>
                    ) : (
                        <div className="workspace-settings-popup danger pane">
                            <div className="workspace-setting-popup">
                                <h2 className="sub-heading"> Transfer ownership</h2>
                                <SelectMenu
                                    options={this.state.transferList}
                                    theme={"red"}
                                    value={this.state.selectedUser}
                                    onChange={(type) => {
                                        this.setState({ selectedUser: type });
                                    }}
                                />
                                <button
                                    className="workspace-settings-red-button button"
                                    onClick={() => {
                                        if (this.state.selectedUser === null) {
                                            toast.error("No user selected");
                                            return;
                                        } else {
                                            this.setState({ selected: true });
                                        }
                                    }}
                                >
                                    Select
                                </button>
                            </div>
                        </div>
                    )}
                </Popup>
            </>
        );
    }
}
