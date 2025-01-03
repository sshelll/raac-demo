permit (
  principal in Role::"SuperAdmin",
  action,
  resource
);

permit (
    principal in Role::"SystemAdmin",
    action,
    resource is SystemSettings
);

permit (
    principal in Role::"HR",
    action,
    resource is Talent
);
