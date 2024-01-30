# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [General Guidance]

**Guiding Principles**

-   Changelogs are for humans, not machines.
-   There should be an entry for every single version.
-   The same types of changes should be grouped.
-   Versions and sections should be linkable.
-   The latest version comes first.
-   The release date of each version is displayed.
-   Mention whether you follow Semantic Versioning.

**Types of changes**

-   `Added` for new features.
-   `Changed` for changes in existing functionality.
-   `Deprecated` for soon-to-be removed features.
-   `Removed` for now removed features.
-   `Fixed` for any bug fixes.
-   `Security` in case of vulnerabilities.

## Semantic Versioning

Example 1.5.16

Breaking down the above example, there has been 1 major break in the API, 5 minor revisions or additions, and 16 bug fixes or resolutions. Please stick to this type of versioning.

# [Unreleased]

## [1.3.3] - 2024-01-30

### Added 

- You now have the abbility to commit changes to the registry via the UI!
- Once you submit your changes, the submit button changes to submitted and disabled.
- Once you edit values, the Submit button changes from submitted to submit and re-enables.

### Changed

- The default behavior when the application loads is that you have submitted the data already. This is because it is pulling from the registry.

### Fixed

- Fixed a bug where once you submitted the data, and then edited it again, the button retained a bad state.

# [Released]

## [1.0.1] - 2024-01-26

### Added

-   New functioning UI with SLINT.
-   Admin Rights Detection.

### Changed

-   Breaking CLI Arguments.
-   Changed UI library.

### Fixed

-   Fixed Bug that displayed an innapropriate default value to the client.
