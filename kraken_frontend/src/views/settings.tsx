import React from "react";
import { toast } from "react-toastify";
import { Api } from "../api/api";
import { FullOauthClient, FullWordlist, SettingsFull } from "../api/generated";
import { alertReact, alertText, promptInput } from "../components/dialog";
import Input from "../components/input";
import Textarea from "../components/textarea";
import "../styling/settings.css";
import AnonymousIcon from "../svg/anonymous";
import CloseIcon from "../svg/close";
import CopyIcon from "../svg/copy";
import { copyToClipboard, handleApiError } from "../utils/helper";

type SettingsProps = {};
type SettingsState = {
    settings: SettingsFull | null;
    oauthApplications: Array<FullOauthClient>;
    newOAuthAppName: string;
    newOAuthAppRedirectUrl: string;
    wordlists: Array<FullWordlist>;
    wordlistName: string;
    wordlistPath: string;
    wordlistDescription: string;
};

export default class Settings extends React.Component<SettingsProps, SettingsState> {
    constructor(props: SettingsProps) {
        super(props);

        this.state = {
            settings: null,
            oauthApplications: [],
            wordlists: [],
            newOAuthAppName: "",
            newOAuthAppRedirectUrl: "",
            wordlistName: "",
            wordlistPath: "",
            wordlistDescription: "",
        };
    }

    componentDidMount() {
        Promise.all([this.getOAuthApps(), this.retrieveSettings(), this.updateWordlists()]).then();
    }

    async retrieveSettings() {
        await Api.admin.settings.get().then(handleApiError((settings) => this.setState({ settings })));
    }

    async getOAuthApps() {
        await Api.admin.oauthApplications.all().then(
            handleApiError((apps) => {
                this.setState({ oauthApplications: apps.apps });
            }),
        );
    }

    async updateWordlists() {
        await Api.admin.wordlists.all().then(
            handleApiError((wordlists) => {
                this.setState({ wordlists: wordlists.wordlists });
            }),
        );
    }

    async createWordlist() {
        if (this.state.wordlistName === "") {
            toast.error("Name of the wordlist must not be empty");
            return;
        }

        if (this.state.wordlistPath === "") {
            toast.error("Path of the wordlist must not be empty");
        }

        await Api.admin.wordlists
            .create({
                name: this.state.wordlistName,
                path: this.state.wordlistPath,
                description: this.state.wordlistDescription,
            })
            .then(handleApiError((_) => toast.success("Created wordlist")));
    }

    async saveSettings() {
        let { settings } = this.state;

        if (settings === null) {
            return;
        }

        await Api.admin.settings.update(settings).then(handleApiError((_) => toast.success("Settings updated")));
    }

    async createOAuthApp() {
        let { newOAuthAppName, newOAuthAppRedirectUrl } = this.state;
        if (newOAuthAppName === "" || newOAuthAppRedirectUrl === "") {
            toast.error("App name and redirect uri must not be empty");
            return;
        }

        await Api.admin.oauthApplications
            .create({ name: newOAuthAppName, redirectUri: newOAuthAppRedirectUrl })
            .then(handleApiError((_) => toast.success("OAuth application created")));
    }

    render() {
        return (
            <>
                <div className={"settings-container"}>
                    <div className={"settings-heading pane"}>
                        <h1 className={"heading"}>Kraken Settings</h1>
                    </div>
                    {this.state.settings !== null ? (
                        <form
                            className={"settings-dehashed pane"}
                            method={"post"}
                            onSubmit={async (x) => {
                                x.preventDefault();
                                await this.saveSettings();
                            }}
                        >
                            <h2 className={"heading"}>Dehashed API</h2>
                            <Input
                                value={
                                    this.state.settings.dehashedEmail !== null &&
                                    this.state.settings.dehashedEmail !== undefined
                                        ? this.state.settings.dehashedEmail
                                        : ""
                                }
                                onChange={(v) => {
                                    let { settings } = this.state;
                                    if (settings !== null) {
                                        settings.dehashedEmail = v;
                                    }
                                    this.setState({ settings });
                                }}
                                placeholder={"email"}
                            />
                            <Input
                                value={
                                    this.state.settings.dehashedApiKey !== null &&
                                    this.state.settings.dehashedApiKey !== undefined
                                        ? this.state.settings.dehashedApiKey
                                        : ""
                                }
                                onChange={(v) => {
                                    let { settings } = this.state;
                                    if (settings !== null) {
                                        settings.dehashedApiKey = v;
                                    }
                                    this.setState({ settings });
                                }}
                                placeholder={"api-key"}
                            />
                            <button className={"button"}>Save</button>
                        </form>
                    ) : (
                        <div className={"pane"}>Loading ...</div>
                    )}
                    <div className={"pane settings-wordlists"}>
                        <h2 className={"heading"}>Wordlists</h2>
                        <form
                            className={"settings-wordlists-creation"}
                            method={"post"}
                            onSubmit={async (event) => {
                                event.preventDefault();
                                await this.createWordlist();
                                await this.updateWordlists();
                            }}
                        >
                            <h3 className={"sub-heading"}>Create wordlist</h3>
                            <label htmlFor={"wordlist-name"}>Name</label>
                            <Input
                                id={"wordlist-name"}
                                required={true}
                                value={this.state.wordlistName}
                                onChange={(v) => this.setState({ wordlistName: v })}
                            />
                            <label htmlFor={"wordlist-path"}>Path</label>
                            <Input
                                id={"wordlist-path"}
                                required={true}
                                value={this.state.wordlistPath}
                                onChange={(v) => this.setState({ wordlistPath: v })}
                            />
                            <label htmlFor={"wordlist-description"}>Description</label>
                            <Textarea
                                id={"wordlist-description"}
                                value={this.state.wordlistDescription}
                                onChange={(v) => this.setState({ wordlistDescription: v })}
                            />
                            <button className={"button"}>Create</button>
                        </form>
                        <h3 className={"sub-heading"}>Existing wordlists</h3>
                        <div className={"settings-wordlists-list"}>
                            <span>Name</span>
                            <span>Path</span>
                            <span>Description</span>
                            <span>Delete</span>
                            {this.state.wordlists.map((x) => (
                                <>
                                    <span>{x.name}</span>
                                    <span>{x.path}</span>
                                    <span>{x.description}</span>
                                    <button
                                        className={"icon-button"}
                                        onClick={async () => {
                                            await Api.admin.wordlists.delete(x.uuid).then(
                                                handleApiError(async () => {
                                                    toast.success("Wordlist deleted");
                                                    await this.updateWordlists();
                                                }),
                                            );
                                        }}
                                    >
                                        <CloseIcon />
                                    </button>
                                </>
                            ))}
                        </div>
                    </div>
                    <div className={"pane settings-oauth"}>
                        <h2 className={"heading"}>OAuth applications</h2>
                        <form
                            className={"settings-oauth-app-creation"}
                            method={"post"}
                            onSubmit={async (event) => {
                                event.preventDefault();
                                await this.createOAuthApp();
                                await this.getOAuthApps();
                            }}
                        >
                            <h3 className={"sub-heading"}>Create app</h3>
                            <Input
                                placeholder={"app name"}
                                value={this.state.newOAuthAppName}
                                onChange={(v) => this.setState({ newOAuthAppName: v })}
                            />
                            <Input
                                placeholder={"redirect url"}
                                value={this.state.newOAuthAppRedirectUrl}
                                onChange={(v) => this.setState({ newOAuthAppRedirectUrl: v })}
                            />
                            <button className={"button"}>Create</button>
                        </form>

                        <h3 className={"sub-heading"}>Existing apps</h3>
                        <div className={"settings-oauth-applications"}>
                            <div className={"settings-oauth-applications-row"}>
                                <div>Name</div>
                                <div>Redirect URL</div>
                                <div>Debug Flow</div>
                                <div>Client ID</div>
                                <div>Secret Key</div>
                                <div>Delete</div>
                            </div>

                            {this.state.oauthApplications.map((x) => (
                                <div key={x.uuid} className={"settings-oauth-applications-row"}>
                                    <div>{x.name}</div>
                                    <span>{x.redirectUri}</span>
                                    <button
                                        className={"icon-button"}
                                        onClick={async () => {
                                            try {
                                                await alertText("Info", "This function allows to test out the OAuth login flow and get a temporary OAuth token for use with the external API.");
                                                let workspaceUuid = await promptInput("Enter allowed workspace UUID");
                                                if (!workspaceUuid)
                                                    return;
                                                await alertText("Info", "A new tab will open now that prompts to authenticate for this app. After allowing access, copy the ID from the URL and paste it here.");
                                                window.open("/api/v1/oauth/auth?response_type=code"
                                                    + "&client_id=" + encodeURIComponent(x.uuid)
                                                    + "&scope=" + encodeURIComponent("workspace/" + workspaceUuid.trim())
                                                    + "&redirect_uri=" + encodeURIComponent(x.redirectUri)
                                                    + "&state=1"
                                                    // DON'T copy this for production OAuth login, this is only for testing!
                                                    + "&code_challenge=-4cf-Mzo_qg9-uq0F4QwWhRh4AjcAqNx7SbYVsdmyQM"
                                                    + "&code_challenge_method=S256",
                                                    "_blank");
                                                let oauthId = await promptInput("Enter OAuth ID from URL");
                                                if (!oauthId)
                                                    return;
                                                let res = await fetch("/api/v1/oauth-server/token", {
                                                    headers: {
                                                        "Content-Type": "application/x-www-form-urlencoded"
                                                    },
                                                    method: "POST",
                                                    body: "grant_type=authorization_code"
                                                        + "&code=" + encodeURIComponent(oauthId.trim())
                                                        + "&redirect_uri=" + encodeURIComponent(x.redirectUri)
                                                        + "&client_id=" + encodeURIComponent(x.uuid)
                                                        + "&client_secret=" + encodeURIComponent(x.secret)
                                                        + "&code_verifier=01234567890123456789012345678901234567890123456789"
                                                });
                                                await alertReact("OAuth Result", (<pre>{JSON.stringify(await res.json(), null, "  ")}</pre>));
                                            } catch (e) {
                                                alertText("Failed", "" + e);
                                                console.error(e);
                                            }
                                        }}
                                    >
                                        <AnonymousIcon />
                                    </button>
                                    <button
                                        className={"icon-button"}
                                        onClick={async () => {
                                            await copyToClipboard(x.uuid);
                                        }}
                                    >
                                        <CopyIcon />
                                    </button>
                                    <button
                                        className={"icon-button"}
                                        onClick={async () => {
                                            await copyToClipboard(x.secret);
                                        }}
                                    >
                                        <CopyIcon />
                                    </button>
                                    <button
                                        className={"icon-button"}
                                        onClick={async () => {
                                            await Api.admin.oauthApplications.delete(x.uuid).then(
                                                handleApiError(async (_) => {
                                                    toast.success(`Deleted application ${x.name}`);
                                                    await this.getOAuthApps();
                                                }),
                                            );
                                        }}
                                    >
                                        <CloseIcon />
                                    </button>
                                </div>
                            ))}
                        </div>
                    </div>
                </div>
            </>
        );
    }
}
