use clap::ArgMatches;
use crate::client::{ApiClient, ApiError, Sendable};
use serde::{Serialize, Deserialize};

use std::io::Read;

pub(crate) fn read_from_input<T>(matches: Option<&ArgMatches<'_>>) -> Result<T, crate::ClientError>
where
    T: Serialize,
    for<'de> T: Deserialize<'de>
{
    let path = matches
        .expect("no args for builder with body?")
        .value_of("payload").expect("payload?");

    let mut bytes = vec![];
    if path == "-" {
        std::io::stdin().read_to_end(&mut bytes).map_err(crate::ClientError::Io)?;
    } else {
        std::fs::File::open(&path)
            .and_then(|mut fd| fd.read_to_end(&mut bytes))
            .map_err(crate::ClientError::Io)?;
    };

    let err = match serde_json::from_reader(bytes.as_slice()) {
        Ok(t) => return Ok(t),
        Err(e) => crate::ClientError::Api(ApiError::ApplicationJson(e)),
    };

    log::debug!("Error decoding payload as application/json: {:?}", err);

    let err = match serde_yaml::from_reader(bytes.as_slice()) {
        Ok(t) => return Ok(t),
        Err(e) => crate::ClientError::Api(ApiError::ApplicationYaml(e)),
    };

    log::debug!("Error decoding payload as application/yaml: {:?}", err);

    Err(err)
}

pub(super) async fn fetch_response<'a, C>(client: &'a C,
                                          _matches: &ArgMatches<'_>,
                                          sub_cmd: &str,
                                          sub_matches: Option<&ArgMatches<'_>>)
                                          -> Result<C::Response, crate::ClientError>
where
    C: ApiClient + Send + Sync + 'static,
    crate::ClientError: From<ApiError<C::Response>>
{
    let resp = match sub_cmd {

        "issue-post-comment-reaction" => {
            let builder = crate::edit_reaction_option::EditReactionOptionPostBuilder::from_args(sub_matches)?;
            builder.send_raw(client).await
        },
        "issue-delete-comment-reaction" => {
            let builder = crate::edit_reaction_option::EditReactionOptionDeleteBuilder::from_args(sub_matches)?;
            builder.send_raw(client).await
        },
        "issue-post-issue-reaction" => {
            let builder = crate::edit_reaction_option::EditReactionOptionPostBuilder1::from_args(sub_matches)?;
            builder.send_raw(client).await
        },
        "issue-delete-issue-reaction" => {
            let builder = crate::edit_reaction_option::EditReactionOptionDeleteBuilder1::from_args(sub_matches)?;
            builder.send_raw(client).await
        },
        "repo-get-contents-list" => {
            let builder = crate::contents_response::ContentsResponseGetBuilder::from_args(sub_matches)?;
            builder.send_raw(client).await
        },
        "repo-get-contents" => {
            let builder = crate::contents_response::ContentsResponseGetBuilder1::from_args(sub_matches)?;
            builder.send_raw(client).await
        },
        "repo-add-collaborator" => {
            let builder = crate::add_collaborator_option::AddCollaboratorOptionPutBuilder::from_args(sub_matches)?;
            builder.send_raw(client).await
        },
        "issue-create-comment" => {
            let builder = crate::create_issue_comment_option::CreateIssueCommentOptionPostBuilder::from_args(sub_matches)?;
            builder.send_raw(client).await
        },
        "user-delete-email" => {
            let builder = crate::delete_email_option::DeleteEmailOptionDeleteBuilder::from_args(sub_matches)?;
            builder.send_raw(client).await
        },
        "repo-merge-pull-request" => {
            let builder = crate::merge_pull_request_option::MergePullRequestOptionPostBuilder::from_args(sub_matches)?;
            builder.send_raw(client).await
        },
        "repo-list-all-git-refs" => {
            let builder = crate::reference::ReferenceGetBuilder::from_args(sub_matches)?;
            builder.send_raw(client).await
        },
        "repo-list-git-refs" => {
            let builder = crate::reference::ReferenceGetBuilder1::from_args(sub_matches)?;
            builder.send_raw(client).await
        },
        "repo-get-all-commits" => {
            let builder = crate::commit::CommitGetBuilder::from_args(sub_matches)?;
            builder.send_raw(client).await
        },
        "repo-get-single-commit" => {
            let builder = crate::commit::CommitGetBuilder1::from_args(sub_matches)?;
            builder.send_raw(client).await
        },
        "user-current-list-gpg-keys" => {
            let builder = crate::gpg_key::GpgKeyGetBuilder::from_args(sub_matches)?;
            builder.send_raw(client).await
        },
        "user-current-get-gpg-key" => {
            let builder = crate::gpg_key::GpgKeyGetBuilder1::from_args(sub_matches)?;
            builder.send_raw(client).await
        },
        "user-list-gpg-keys" => {
            let builder = crate::gpg_key::GpgKeyGetBuilder2::from_args(sub_matches)?;
            builder.send_raw(client).await
        },
        "repo-list-tags" => {
            let builder = crate::tag::TagGetBuilder::from_args(sub_matches)?;
            builder.send_raw(client).await
        },
        "repo-search" => {
            let builder = crate::search_results::SearchResultsGetBuilder::from_args(sub_matches)?;
            builder.send_raw(client).await
        },
        "admin-create-public-key" => {
            let builder = crate::create_key_option::CreateKeyOptionPostBuilder::from_args(sub_matches)?;
            builder.send_raw(client).await
        },
        "repo-create-key" => {
            let builder = crate::create_key_option::CreateKeyOptionPostBuilder1::from_args(sub_matches)?;
            builder.send_raw(client).await
        },
        "user-current-post-key" => {
            let builder = crate::create_key_option::CreateKeyOptionPostBuilder2::from_args(sub_matches)?;
            builder.send_raw(client).await
        },
        "repo-edit-release-attachment" => {
            let builder = crate::edit_attachment_options::EditAttachmentOptionsPatchBuilder::from_args(sub_matches)?;
            builder.send_raw(client).await
        },
        "repo-update-topics" => {
            let builder = crate::repo_topic_options::RepoTopicOptionsPutBuilder::from_args(sub_matches)?;
            builder.send_raw(client).await
        },
        "team-search" => {
            let builder = crate::get_orgs_org_teams_search_response::GetOrgsOrgTeamsSearchResponseGetBuilder::from_args(sub_matches)?;
            builder.send_raw(client).await
        },
        "user-add-email" => {
            let builder = crate::create_email_option::CreateEmailOptionPostBuilder::from_args(sub_matches)?;
            builder.send_raw(client).await
        },
        "create-fork" => {
            let builder = crate::create_fork_option::CreateForkOptionPostBuilder::from_args(sub_matches)?;
            builder.send_raw(client).await
        },
        "issue-edit-milestone" => {
            let builder = crate::edit_milestone_option::EditMilestoneOptionPatchBuilder::from_args(sub_matches)?;
            builder.send_raw(client).await
        },
        "get-blob" => {
            let builder = crate::git_blob_response::GitBlobResponseGetBuilder::from_args(sub_matches)?;
            builder.send_raw(client).await
        },
        "org-edit-hook" => {
            let builder = crate::edit_hook_option::EditHookOptionPatchBuilder::from_args(sub_matches)?;
            builder.send_raw(client).await
        },
        "repo-edit-hook" => {
            let builder = crate::edit_hook_option::EditHookOptionPatchBuilder1::from_args(sub_matches)?;
            builder.send_raw(client).await
        },
        "repo-edit" => {
            let builder = crate::edit_repo_option::EditRepoOptionPatchBuilder::from_args(sub_matches)?;
            builder.send_raw(client).await
        },
        "repo-list-keys" => {
            let builder = crate::deploy_key::DeployKeyGetBuilder::from_args(sub_matches)?;
            builder.send_raw(client).await
        },
        "repo-get-key" => {
            let builder = crate::deploy_key::DeployKeyGetBuilder1::from_args(sub_matches)?;
            builder.send_raw(client).await
        },
        "org-edit-team" => {
            let builder = crate::edit_team_option::EditTeamOptionPatchBuilder::from_args(sub_matches)?;
            builder.send_raw(client).await
        },
        "get-tree" => {
            let builder = crate::git_tree_response::GitTreeResponseGetBuilder::from_args(sub_matches)?;
            builder.send_raw(client).await
        },
        "org-list-teams" => {
            let builder = crate::team::TeamGetBuilder::from_args(sub_matches)?;
            builder.send_raw(client).await
        },
        "org-get-team" => {
            let builder = crate::team::TeamGetBuilder1::from_args(sub_matches)?;
            builder.send_raw(client).await
        },
        "user-list-teams" => {
            let builder = crate::team::TeamGetBuilder2::from_args(sub_matches)?;
            builder.send_raw(client).await
        },
        "get-version" => {
            let builder = crate::server_version::ServerVersionGetBuilder::from_args(sub_matches)?;
            builder.send_raw(client).await
        },
        "repo-list-topics" => {
            let builder = crate::topic_name::TopicNameGetBuilder::from_args(sub_matches)?;
            builder.send_raw(client).await
        },
        "admin-create-org" => {
            let builder = crate::create_org_option::CreateOrgOptionPostBuilder::from_args(sub_matches)?;
            builder.send_raw(client).await
        },
        "org-create" => {
            let builder = crate::create_org_option::CreateOrgOptionPostBuilder1::from_args(sub_matches)?;
            builder.send_raw(client).await
        },
        "user-get-tokens" => {
            let builder = crate::access_token::AccessTokenGetBuilder::from_args(sub_matches)?;
            builder.send_raw(client).await
        },
        "repo-update-file" => {
            let builder = crate::update_file_options::UpdateFileOptionsPutBuilder::from_args(sub_matches)?;
            builder.send_raw(client).await
        },
        "user-get-heatmap-data" => {
            let builder = crate::user_heatmap_data::UserHeatmapDataGetBuilder::from_args(sub_matches)?;
            builder.send_raw(client).await
        },
        "issue-get-repo-comments" => {
            let builder = crate::comment::CommentGetBuilder::from_args(sub_matches)?;
            builder.send_raw(client).await
        },
        "issue-get-comments" => {
            let builder = crate::comment::CommentGetBuilder1::from_args(sub_matches)?;
            builder.send_raw(client).await
        },
        "user-search" => {
            let builder = crate::get_users_search_response::GetUsersSearchResponseGetBuilder::from_args(sub_matches)?;
            builder.send_raw(client).await
        },
        "user-current-check-subscription" => {
            let builder = crate::watch_info::WatchInfoGetBuilder::from_args(sub_matches)?;
            builder.send_raw(client).await
        },
        "user-current-put-subscription" => {
            let builder = crate::watch_info::WatchInfoPutBuilder::from_args(sub_matches)?;
            builder.send_raw(client).await
        },
        "repo-get-combined-status-by-ref" => {
            let builder = crate::status::StatusGetBuilder::from_args(sub_matches)?;
            builder.send_raw(client).await
        },
        "repo-list-statuses" => {
            let builder = crate::status::StatusGetBuilder1::from_args(sub_matches)?;
            builder.send_raw(client).await
        },
        "org-create-hook" => {
            let builder = crate::create_hook_option::CreateHookOptionPostBuilder::from_args(sub_matches)?;
            builder.send_raw(client).await
        },
        "repo-create-hook" => {
            let builder = crate::create_hook_option::CreateHookOptionPostBuilder1::from_args(sub_matches)?;
            builder.send_raw(client).await
        },
        "org-edit" => {
            let builder = crate::edit_org_option::EditOrgOptionPatchBuilder::from_args(sub_matches)?;
            builder.send_raw(client).await
        },
        "admin-get-all-orgs" => {
            let builder = crate::organization::OrganizationGetBuilder::from_args(sub_matches)?;
            builder.send_raw(client).await
        },
        "org-get" => {
            let builder = crate::organization::OrganizationGetBuilder1::from_args(sub_matches)?;
            builder.send_raw(client).await
        },
        "org-list-current-user-orgs" => {
            let builder = crate::organization::OrganizationGetBuilder2::from_args(sub_matches)?;
            builder.send_raw(client).await
        },
        "org-list-user-orgs" => {
            let builder = crate::organization::OrganizationGetBuilder3::from_args(sub_matches)?;
            builder.send_raw(client).await
        },
        "issue-create-label" => {
            let builder = crate::create_label_option::CreateLabelOptionPostBuilder::from_args(sub_matches)?;
            builder.send_raw(client).await
        },
        "admin-get-all-users" => {
            let builder = crate::user::UserGetBuilder::from_args(sub_matches)?;
            builder.send_raw(client).await
        },
        "org-list-members" => {
            let builder = crate::user::UserGetBuilder1::from_args(sub_matches)?;
            builder.send_raw(client).await
        },
        "org-list-public-members" => {
            let builder = crate::user::UserGetBuilder2::from_args(sub_matches)?;
            builder.send_raw(client).await
        },
        "repo-list-collaborators" => {
            let builder = crate::user::UserGetBuilder3::from_args(sub_matches)?;
            builder.send_raw(client).await
        },
        "issue-subscriptions" => {
            let builder = crate::user::UserGetBuilder4::from_args(sub_matches)?;
            builder.send_raw(client).await
        },
        "repo-list-stargazers" => {
            let builder = crate::user::UserGetBuilder5::from_args(sub_matches)?;
            builder.send_raw(client).await
        },
        "repo-list-subscribers" => {
            let builder = crate::user::UserGetBuilder6::from_args(sub_matches)?;
            builder.send_raw(client).await
        },
        "org-list-team-members" => {
            let builder = crate::user::UserGetBuilder7::from_args(sub_matches)?;
            builder.send_raw(client).await
        },
        "org-list-team-member" => {
            let builder = crate::user::UserGetBuilder8::from_args(sub_matches)?;
            builder.send_raw(client).await
        },
        "user-get-current" => {
            let builder = crate::user::UserGetBuilder9::from_args(sub_matches)?;
            builder.send_raw(client).await
        },
        "user-current-list-followers" => {
            let builder = crate::user::UserGetBuilder10::from_args(sub_matches)?;
            builder.send_raw(client).await
        },
        "user-current-list-following" => {
            let builder = crate::user::UserGetBuilder11::from_args(sub_matches)?;
            builder.send_raw(client).await
        },
        "user-get" => {
            let builder = crate::user::UserGetBuilder12::from_args(sub_matches)?;
            builder.send_raw(client).await
        },
        "user-list-followers" => {
            let builder = crate::user::UserGetBuilder13::from_args(sub_matches)?;
            builder.send_raw(client).await
        },
        "user-list-following" => {
            let builder = crate::user::UserGetBuilder14::from_args(sub_matches)?;
            builder.send_raw(client).await
        },
        "issue-edit-comment" => {
            let builder = crate::edit_issue_comment_option::EditIssueCommentOptionPatchBuilder::from_args(sub_matches)?;
            builder.send_raw(client).await
        },
        "issue-edit-comment-deprecated" => {
            let builder = crate::edit_issue_comment_option::EditIssueCommentOptionPatchBuilder1::from_args(sub_matches)?;
            builder.send_raw(client).await
        },
        "repo-create-file" => {
            let builder = crate::create_file_options::CreateFileOptionsPostBuilder::from_args(sub_matches)?;
            builder.send_raw(client).await
        },
        "repo-migrate" => {
            let builder = crate::migrate_repo_form::MigrateRepoFormPostBuilder::from_args(sub_matches)?;
            builder.send_raw(client).await
        },
        "org-list-hooks" => {
            let builder = crate::hook::HookGetBuilder::from_args(sub_matches)?;
            builder.send_raw(client).await
        },
        "org-get-hook" => {
            let builder = crate::hook::HookGetBuilder1::from_args(sub_matches)?;
            builder.send_raw(client).await
        },
        "repo-list-hooks" => {
            let builder = crate::hook::HookGetBuilder2::from_args(sub_matches)?;
            builder.send_raw(client).await
        },
        "repo-get-hook" => {
            let builder = crate::hook::HookGetBuilder3::from_args(sub_matches)?;
            builder.send_raw(client).await
        },
        "issue-get-milestones-list" => {
            let builder = crate::milestone::MilestoneGetBuilder::from_args(sub_matches)?;
            builder.send_raw(client).await
        },
        "issue-get-milestone" => {
            let builder = crate::milestone::MilestoneGetBuilder1::from_args(sub_matches)?;
            builder.send_raw(client).await
        },
        "user-current-list-keys" => {
            let builder = crate::public_key::PublicKeyGetBuilder::from_args(sub_matches)?;
            builder.send_raw(client).await
        },
        "user-current-get-key" => {
            let builder = crate::public_key::PublicKeyGetBuilder1::from_args(sub_matches)?;
            builder.send_raw(client).await
        },
        "user-list-keys" => {
            let builder = crate::public_key::PublicKeyGetBuilder2::from_args(sub_matches)?;
            builder.send_raw(client).await
        },
        "topic-search" => {
            let builder = crate::topic_response::TopicResponseGetBuilder::from_args(sub_matches)?;
            builder.send_raw(client).await
        },
        "render-markdown" => {
            let builder = crate::markdown_option::MarkdownOptionPostBuilder::from_args(sub_matches)?;
            builder.send_raw(client).await
        },
        "issue-replace-labels" => {
            let builder = crate::issue_labels_option::IssueLabelsOptionPutBuilder::from_args(sub_matches)?;
            builder.send_raw(client).await
        },
        "issue-add-label" => {
            let builder = crate::issue_labels_option::IssueLabelsOptionPostBuilder::from_args(sub_matches)?;
            builder.send_raw(client).await
        },
        "admin-edit-user" => {
            let builder = crate::edit_user_option::EditUserOptionPatchBuilder::from_args(sub_matches)?;
            builder.send_raw(client).await
        },
        "repo-list-git-hooks" => {
            let builder = crate::git_hook::GitHookGetBuilder::from_args(sub_matches)?;
            builder.send_raw(client).await
        },
        "repo-get-git-hook" => {
            let builder = crate::git_hook::GitHookGetBuilder1::from_args(sub_matches)?;
            builder.send_raw(client).await
        },
        "issue-create-milestone" => {
            let builder = crate::create_milestone_option::CreateMilestoneOptionPostBuilder::from_args(sub_matches)?;
            builder.send_raw(client).await
        },
        "repo-create-status" => {
            let builder = crate::create_status_option::CreateStatusOptionPostBuilder::from_args(sub_matches)?;
            builder.send_raw(client).await
        },
        "repo-list-pull-requests" => {
            let builder = crate::pull_request::PullRequestGetBuilder::from_args(sub_matches)?;
            builder.send_raw(client).await
        },
        "repo-get-pull-request" => {
            let builder = crate::pull_request::PullRequestGetBuilder1::from_args(sub_matches)?;
            builder.send_raw(client).await
        },
        "repo-create-release" => {
            let builder = crate::create_release_option::CreateReleaseOptionPostBuilder::from_args(sub_matches)?;
            builder.send_raw(client).await
        },
        "repo-edit-pull-request" => {
            let builder = crate::edit_pull_request_option::EditPullRequestOptionPatchBuilder::from_args(sub_matches)?;
            builder.send_raw(client).await
        },
        "get-tag" => {
            let builder = crate::annotated_tag::AnnotatedTagGetBuilder::from_args(sub_matches)?;
            builder.send_raw(client).await
        },
        "issue-edit-label" => {
            let builder = crate::edit_label_option::EditLabelOptionPatchBuilder::from_args(sub_matches)?;
            builder.send_raw(client).await
        },
        "user-current-post-gpg-key" => {
            let builder = crate::create_gpg_key_option::CreateGpgKeyOptionPostBuilder::from_args(sub_matches)?;
            builder.send_raw(client).await
        },
        "issue-tracked-times" => {
            let builder = crate::tracked_time::TrackedTimeGetBuilder::from_args(sub_matches)?;
            builder.send_raw(client).await
        },
        "repo-tracked-times" => {
            let builder = crate::tracked_time::TrackedTimeGetBuilder1::from_args(sub_matches)?;
            builder.send_raw(client).await
        },
        "user-tracked-times" => {
            let builder = crate::tracked_time::TrackedTimeGetBuilder2::from_args(sub_matches)?;
            builder.send_raw(client).await
        },
        "user-current-tracked-times" => {
            let builder = crate::tracked_time::TrackedTimeGetBuilder3::from_args(sub_matches)?;
            builder.send_raw(client).await
        },
        "repo-signing-key" => {
            let builder = crate::miscellaneous::MiscellaneousGetBuilder1::from_args(sub_matches)?;
            builder.send_raw(client).await
        },
        "get-signing-key" => {
            let builder = crate::miscellaneous::MiscellaneousGetBuilder2::from_args(sub_matches)?;
            builder.send_raw(client).await
        },
        "repo-list-release-attachments" => {
            let builder = crate::attachment::AttachmentGetBuilder::from_args(sub_matches)?;
            builder.send_raw(client).await
        },
        "repo-create-release-attachment" => {
            let builder = crate::attachment::AttachmentPostBuilder::from_args(sub_matches)?;
            builder.send_raw(client).await
        },
        "repo-get-release-attachment" => {
            let builder = crate::attachment::AttachmentGetBuilder1::from_args(sub_matches)?;
            builder.send_raw(client).await
        },
        "repo-create-pull-request" => {
            let builder = crate::create_pull_request_option::CreatePullRequestOptionPostBuilder::from_args(sub_matches)?;
            builder.send_raw(client).await
        },
        "issue-edit-issue" => {
            let builder = crate::edit_issue_option::EditIssueOptionPatchBuilder::from_args(sub_matches)?;
            builder.send_raw(client).await
        },
        "issue-create-issue" => {
            let builder = crate::create_issue_option::CreateIssueOptionPostBuilder::from_args(sub_matches)?;
            builder.send_raw(client).await
        },
        "repo-list-branches" => {
            let builder = crate::branch::BranchGetBuilder::from_args(sub_matches)?;
            builder.send_raw(client).await
        },
        "repo-get-branch" => {
            let builder = crate::branch::BranchGetBuilder1::from_args(sub_matches)?;
            builder.send_raw(client).await
        },
        "issue-get-labels" => {
            let builder = crate::label::LabelGetBuilder::from_args(sub_matches)?;
            builder.send_raw(client).await
        },
        "issue-list-labels" => {
            let builder = crate::label::LabelGetBuilder1::from_args(sub_matches)?;
            builder.send_raw(client).await
        },
        "issue-get-label" => {
            let builder = crate::label::LabelGetBuilder2::from_args(sub_matches)?;
            builder.send_raw(client).await
        },
        "user-create-token" => {
            let builder = crate::post_users_username_tokens_body::PostUsersUsernameTokensBodyPostBuilder::from_args(sub_matches)?;
            builder.send_raw(client).await
        },
        "issue-get-comment-reactions" => {
            let builder = crate::reaction_response::ReactionResponseGetBuilder::from_args(sub_matches)?;
            builder.send_raw(client).await
        },
        "issue-get-issue-reactions" => {
            let builder = crate::reaction_response::ReactionResponseGetBuilder1::from_args(sub_matches)?;
            builder.send_raw(client).await
        },
        "user-get-stop-watches" => {
            let builder = crate::stop_watch::StopWatchGetBuilder::from_args(sub_matches)?;
            builder.send_raw(client).await
        },
        "repo-list-releases" => {
            let builder = crate::release::ReleaseGetBuilder::from_args(sub_matches)?;
            builder.send_raw(client).await
        },
        "repo-get-release" => {
            let builder = crate::release::ReleaseGetBuilder1::from_args(sub_matches)?;
            builder.send_raw(client).await
        },
        "repo-delete-file" => {
            let builder = crate::delete_file_options::DeleteFileOptionsDeleteBuilder::from_args(sub_matches)?;
            builder.send_raw(client).await
        },
        "repo-edit-git-hook" => {
            let builder = crate::edit_git_hook_option::EditGitHookOptionPatchBuilder::from_args(sub_matches)?;
            builder.send_raw(client).await
        },
        "issue-add-time" => {
            let builder = crate::add_time_option::AddTimeOptionPostBuilder::from_args(sub_matches)?;
            builder.send_raw(client).await
        },
        "admin-create-user" => {
            let builder = crate::create_user_option::CreateUserOptionPostBuilder::from_args(sub_matches)?;
            builder.send_raw(client).await
        },
        "issue-edit-issue-deadline" => {
            let builder = crate::edit_deadline_option::EditDeadlineOptionPostBuilder::from_args(sub_matches)?;
            builder.send_raw(client).await
        },
        "org-create-team" => {
            let builder = crate::create_team_option::CreateTeamOptionPostBuilder::from_args(sub_matches)?;
            builder.send_raw(client).await
        },
        "repo-edit-release" => {
            let builder = crate::edit_release_option::EditReleaseOptionPatchBuilder::from_args(sub_matches)?;
            builder.send_raw(client).await
        },
        "issue-search-issues" => {
            let builder = crate::issue::IssueGetBuilder::from_args(sub_matches)?;
            builder.send_raw(client).await
        },
        "issue-list-issues" => {
            let builder = crate::issue::IssueGetBuilder1::from_args(sub_matches)?;
            builder.send_raw(client).await
        },
        "issue-get-issue" => {
            let builder = crate::issue::IssueGetBuilder2::from_args(sub_matches)?;
            builder.send_raw(client).await
        },
        "org-list-repos" => {
            let builder = crate::repository::RepositoryGetBuilder::from_args(sub_matches)?;
            builder.send_raw(client).await
        },
        "repo-get" => {
            let builder = crate::repository::RepositoryGetBuilder1::from_args(sub_matches)?;
            builder.send_raw(client).await
        },
        "list-forks" => {
            let builder = crate::repository::RepositoryGetBuilder2::from_args(sub_matches)?;
            builder.send_raw(client).await
        },
        "repo-get-by-id" => {
            let builder = crate::repository::RepositoryGetBuilder3::from_args(sub_matches)?;
            builder.send_raw(client).await
        },
        "org-list-team-repos" => {
            let builder = crate::repository::RepositoryGetBuilder4::from_args(sub_matches)?;
            builder.send_raw(client).await
        },
        "user-current-list-repos" => {
            let builder = crate::repository::RepositoryGetBuilder5::from_args(sub_matches)?;
            builder.send_raw(client).await
        },
        "user-current-list-starred" => {
            let builder = crate::repository::RepositoryGetBuilder6::from_args(sub_matches)?;
            builder.send_raw(client).await
        },
        "user-current-list-subscriptions" => {
            let builder = crate::repository::RepositoryGetBuilder7::from_args(sub_matches)?;
            builder.send_raw(client).await
        },
        "user-list-repos" => {
            let builder = crate::repository::RepositoryGetBuilder8::from_args(sub_matches)?;
            builder.send_raw(client).await
        },
        "user-list-starred" => {
            let builder = crate::repository::RepositoryGetBuilder9::from_args(sub_matches)?;
            builder.send_raw(client).await
        },
        "user-list-subscriptions" => {
            let builder = crate::repository::RepositoryGetBuilder10::from_args(sub_matches)?;
            builder.send_raw(client).await
        },
        "user-list-emails" => {
            let builder = crate::email::EmailGetBuilder::from_args(sub_matches)?;
            builder.send_raw(client).await
        },
        "admin-create-repo" => {
            let builder = crate::create_repo_option::CreateRepoOptionPostBuilder::from_args(sub_matches)?;
            builder.send_raw(client).await
        },
        "create-org-repo" => {
            let builder = crate::create_repo_option::CreateRepoOptionPostBuilder1::from_args(sub_matches)?;
            builder.send_raw(client).await
        },
        "create-current-user-repo" => {
            let builder = crate::create_repo_option::CreateRepoOptionPostBuilder2::from_args(sub_matches)?;
            builder.send_raw(client).await
        },
        _ => unimplemented!(),
    };

    match resp {
        Ok(r) => Ok(r),
        Err(ApiError::Failure(_, _, r)) => Ok(r.into_inner()),
        Err(e) => return Err(e.into()),
    }
}
