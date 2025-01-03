# RAAC-Demo(WIP)

RBAC + ABAC = RAAC, based on [cedar-policy](https://github.com/cedar-policy/cedar), built with Rust.

## Senario

Inspired by my work experience at [Lark](https://hire.feishu.cn/).

Assume that we have a recruitment system, which helps HRs to manage the recruitment process.

There're several:

- roles in the system: `HR`, `SuperAdmin`, `SystemAdmin`, etc.
- core entities in the system: `SystemSetting`, `Talent`(aka the resume of candidate), etc.

And the authentication requirements are like:

- `HR` can modify the `Talent`.
- `SuperAdmin` can do everything.
- `SystemAdmin` can only change the settings of the system.

Seems like a typical RBAC system, right?

### Challenge 1

However, the real challenge comes from the dynamic customization of the system:

- Customers can customize the system by adding new roles.
- Customers can always ignore the predefined roles and choose to define their own roles instead.

How the hell can we allow customers to DIY another `MySuperAdmin` thing, since we've already hardcoded the `SuperAdmin` logic in the system?

In hence, the traditional RBAC system is not enough to meet the requirements. We have to break down the granularity even more:

1. Figure out the abilities of a role, like a `HR` is able to `modify` the `Talent`.
2. Turn this ability into a `action`, like `Talent.modify`.
3. Allow customers to define their own roles by combining these `actions`.

Let us now revisit these two concepts:

- `R(Role)`, which is a collection of `A(Actions)`.
- `A(Actions)`, which is the smallest unit of permission.

---

### Challenge 2

What's more, another challenge occurs when the accessibility depends on the resource itself.

E.g.:

#### Hide something

Maybe you've been granted the permission to `view` a `Talent`, but one day the PM says: "We need to add a new feature to hide a `Talent`", so you add a new attribute `Talent.hide` to the system.

And now you can only view the `Talent` when the `Talent.hide` is `false`.

#### Share something

Several days later, the annoying PM says: "Here comes another brand new feature! We shall allow the powerful users to share a `Talent` to a nobody!"

So you create a new table named `talent_shares`, which links the `Talent` to the `Nobody`.

And now you must be careful when you're trying to reject someone's request to view a `Talent`, because the `Talent` may have been shared to him by a powerful user.
