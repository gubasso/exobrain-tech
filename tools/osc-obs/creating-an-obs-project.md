# Creating an OBS project

> <https://openbuildservice.org/help/manuals/obs-user-guide/> · <https://www.mankier.com/1/osc>

Create a project on an OBS instance from the command line, whether it is a home project or a
subproject under an existing one. Both are the same operation: a subproject is a project whose name
carries a colon, and OBS has no separate subproject object.

## Preconditions

- `osc` is installed (`zypper in osc`, or your distro's package).
- Your account is configured for the target instance. Probe it with
  `osc -A <apiurl> api /person/<user>`, which returns a `<person>` XML body.
- For a name outside `home:<you>`, the maintainer role on the parent project. Step 1 checks this.

## 1. Check you may create the name

```bash
osc -A <apiurl> meta prj <parent-project> | grep -E '<(person|group)'
```

Look for your login, or a group you belong to, with `role="maintainer"`.

| Name you want                           | What OBS requires                                             |
| --------------------------------------- | ------------------------------------------------------------- |
| `home:<you>` or `home:<you>:<anything>` | Nothing; always permitted                                     |
| `<parent>:<child>`                      | `create_project` on the nearest existing ancestor of the name |
| A new top-level name                    | Instance admin; a normal account cannot                       |

The `maintainer` role carries `create_project`; `bugowner` does not. The check walks up the
ancestors, so maintainer on `devel` also covers `devel:languages:python`. The rule is
[`can_create_project?`](https://github.com/openSUSE/open-build-service/blob/master/src/api/app/models/user.rb)
and the role-to-permission seed is
[`db/seeds.rb`](https://github.com/openSUSE/open-build-service/blob/master/src/api/db/seeds.rb).

## 2. Pick the name

Names are colon-separated, and the parent is the longest prefix that already exists:
`devel:languages:python:extra` resolves to `devel:languages:python` if that exists, otherwise
`devel:languages`, otherwise `devel`
([`Project#parent`](https://github.com/openSUSE/open-build-service/blob/master/src/api/app/models/project.rb)).

A parent that does not exist yet is created first, as its own project.

## 3. Write the project meta

Start from [`templates/project-meta.xml`](./templates/project-meta.xml) and change the name, the
person entries, and the repositories:

```xml
<project name="<project>">
  <title>Short project title</title>
  <description>What this project is for.</description>
  <person userid="<you>" role="maintainer"/>
  <person userid="<you>" role="bugowner"/>
  <build>    <enable/></build>
  <publish>  <enable/></publish>
  <debuginfo><enable/></debuginfo>
</project>
```

Leave the repositories out for now; step 5 adds them once the project exists.

## 4. Apply it

```bash
osc -A <apiurl> meta prj -e <project>
```

The editor opens on the metadata and the project is created when you save — creating a resource is
editing one that does not exist yet, and there is no separate create verb.

To apply a file instead, non-interactively:

```bash
osc -A <apiurl> meta prj -F project-meta.xml <project>
```

`-F -` reads from stdin, which is what a bootstrap script uses.

## 5. Add build targets

Edit the meta again and add one `<repository>` per lane you want built:

```xml
<repository name="SLE_15_SP7">
  <path project="SUSE:SLE-15-SP7:Update" repository="pool"/>
  <path project="SUSE:SLE-15-SP7:GA"     repository="standard"/>
  <arch>x86_64</arch>
</repository>
```

The first `<path>` is the resolver's base and later ones satisfy what it did not find. Maintenance
projects publish under `pool` rather than `standard` — see
[sle-update-pool-vs-standard.md](./sle-update-pool-vs-standard.md). Every lane costs scheduler time,
so add the arches and service packs you will actually look at.

## 6. Verify

```bash
osc -A <apiurl> meta prj <project>
```

The command returns your metadata. To list what now exists under a parent:

```bash
osc -A <apiurl> api '/search/project/id?match=starts-with(@name,"<parent-project>:")'
```

`osc ls <parent-project>` answers a different question — it lists packages in that project, not its
subprojects.

## What a subproject does not inherit

| From the parent                     | Inherited                                                |
| ----------------------------------- | -------------------------------------------------------- |
| Repositories and build targets      | No; declare your own                                     |
| Project config (`osc meta prjconf`) | No; set it on the subproject                             |
| Packages                            | No                                                       |
| Maintainer write access             | Yes; parent maintainers can always modify the subproject |

The last row is a namespace, not a private space. A `home:` project is the private one.

## Troubleshooting

| Symptom                             | Cause                              | Fix                                                       |
| ----------------------------------- | ---------------------------------- | --------------------------------------------------------- |
| `403` on save                       | Not maintainer on the parent       | Ask a parent maintainer to add you; do not work around it |
| Creation refused, no parent found   | No ancestor of the name exists     | Create the parent first, or pick an existing prefix       |
| `osc ls <parent>` shows nothing new | It lists packages, not subprojects | Use the `/search/project/id` call in step 6               |
| Nothing builds after creation       | No repository declared             | Add build targets, step 5                                 |

## Web UI equivalent

On the project page, open the Subprojects tab, click New subproject, fill the form, and press Create
project. The permission rule is the same one step 1 checks.
