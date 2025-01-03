// resources
entity Talent;
entity SystemSettings;

// entities
entity Role;
entity User in [Role];

// actions
action any appliesTo {
    principal: [User],
    resource: [Talent, SystemSettings]
};
