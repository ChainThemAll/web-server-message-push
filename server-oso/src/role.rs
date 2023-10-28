use std::fmt;

use oso::PolarClass;

#[derive(Clone, Debug, PartialEq, PolarClass)]
pub enum Role {
    /// Responsible for the setup, configuration and maintenance of the entire system.
    ///Manage user accounts and permissions.
    /// Monitor system performance and security.
    /// Handle problems and failures in the system.
    Administrator,
    ///Responsible for code updates and maintenance.
    /// Solve user feedback issues.
    /// Update and maintain system documentation.
    Maintainer,
    ///Provide code or content contributions.
    ///Participate in project discussions and code reviews.
    Contributor,

    ///Use web service functionality.
    ///Provide feedback and report issues.
    User,
    ///Audit systems for security and compliance.
    ///Examine and assess system risks.
    Auditor,
    ///Provide user support and assistance.
    ///Answer user questions and concerns.
    CustomerService,
    ///Responsible for the daily operation and promotion of the website.
    ///Analyze user data and market trends.
    Operator,
    ///Test system functionality and performance.
    ///Report and track bugs.
    Tester,
    ///Analyze user data and system performance data.
    ///Provide data support and reporting.
    DataAnalyst,
    ///Ensure the project is on track.
    ///Coordinate team members and resources.
    ProjectManager,
}
impl fmt::Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let lowercase_str = match self {
            Role::Administrator => "administrator",
            Role::Maintainer => "maintainer",
            Role::Contributor => "contributor",
            Role::User => "user",
            Role::Auditor => "auditor",
            Role::CustomerService => "customerService",
            Role::Operator => "operator",
            Role::Tester => "tester",
            Role::DataAnalyst => "dataAnalyst",
            Role::ProjectManager => "projectManager",
        };
        write!(f, "{}", lowercase_str)
    }
}
