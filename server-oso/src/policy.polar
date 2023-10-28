allow(user: User, action: String, repository: Repository) if
    action in repository.permissions and (
        (user.role = "administrator" and (action = "read" or action = "write" or action = "create" or action = "delete" or action = "manage" or action = "grant")) or
        (user.role = "maintainer" and (action = "read" or action = "write" or action = "create" or action = "manage")) or
        (user.role = "contributor" and (action = "read" or action = "write" or action = "create")) or
        (user.role = "auditor" and action = "audit") or
        (user.role = "operator" and (action = "read" or action = "write" or action = "execute")) or
        (user.role = "tester" and (action = "read" or action = "write")) or
        (user.role = "dataAnalyst" and action = "read") or
        (user.role = "projectManager" and (action = "read" or action = "write" or action = "create" or action = "manage"))
    );


allow_field(user, "read", profile: Profile, "email") if
    user = profile.user or
    user.is_admin;

allow_request(_, _: Request{path: "/login"});

# Only allow access to payments by users with verified emails
allow_request(user: User, request: Request) if
    request.path.startswith("/payments") and
    user.verified_email;