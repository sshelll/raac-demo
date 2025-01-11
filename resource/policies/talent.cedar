permit (
    principal is User,
    action in [Action::"/talent/view", Action::"/talent/view_hidden"],
    resource is Talent
)
when {
    // can view, if not hidden
    (resource.hide == 0 && principal.atoms.contains("/talent/view")) ||

    // can view hidden, if you have the view hidden atom
    (principal has atoms && principal.atoms.contains("/talent/view_hidden")) ||

    // been shared
    (resource has shared_users && resource.shared_users.contains(principal.user_id))
};
