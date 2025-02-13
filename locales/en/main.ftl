# General
app-name = Flighty
app-description = Flutter Deployment Tool
app-version = v1.0.0

# Init command
init-prompt = How should we refer to this app?
init-default-name = my_flutter_app

# Project name errors
project-name-length-error = Project name must be between 3-30 characters
project-name-start-error = Project name cannot start with a number
project-name-chars-error = Only lowercase letters, numbers and underscores are allowed

# Process steps
step-check-flutter = Checking Flutter project...
step-create-structure = Creating directory structure...
step-update-git = Updating Git configuration...
step-create-config = Creating configuration files...

# Success messages
success-init = Flighty initialized successfully!
success-app-created = Flighty app has been created.
success-config-created = 'flighty.yaml' has been created.
success-pubspec-updated = 'pubspec.yaml' has been updated.

# Command descriptions
commands-intro = Reference the following commands to get started:
command-build = To create a new release
command-deploy = To push an update
command-preview = To preview a release

# Error messages
error-title = Error Details:
error-solution = Solution:
error-flutter-not-found = Flutter project not found
error-flutter-sdk-not-found = Flutter SDK not found
error-structure-failed = Failed to create directory structure
error-git-failed = Failed to update Git configuration
error-config-failed = Failed to create configuration files

# Solution steps
solution-create-project = First create a Flutter project:
solution-install-sdk = Install Flutter SDK:
solution-check-flutter = Check your Flutter installation:

# Documentation
docs-more-info = For more information, visit:

# Commands
cmd-init = Initialize a new project
cmd-init-path = Project directory

cmd-config = View or edit configuration
cmd-config-key = Configuration key
cmd-config-value = New value

cmd-build = Build the application
cmd-build-platform = Build platform (android/ios)
cmd-build-version = Version number
cmd-build-number = Build number
cmd-build-channel = Release channel (alpha/beta/production)

cmd-deploy = Deploy the application
cmd-deploy-platform = Deployment platform (android/ios)
cmd-deploy-target = Deployment target (store/firebase)

cmd-flutter = Manage Flutter SDK
cmd-flutter-command = Flutter command

cmd-language = Change language
cmd-language-code = Language code (en/tr)

# Messages
msg-project-init = Initializing project: {$path}
msg-config-exists = Configuration file already exists
msg-config-created = Default configuration created
msg-flutter-missing = Flutter project not found. Please run 'flutter create' first
msg-build-start = Starting build: {$platform}
msg-build-complete = Build completed: {$platform}
msg-deploy-start = Starting deployment: {$platform} -> {$target}
msg-deploy-complete = Deployment completed: {$platform}
msg-invalid-platform = Invalid platform
msg-invalid-target = Invalid target
msg-invalid-channel = Invalid channel
msg-flutter-not-found = Flutter SDK not found
msg-language-changed = Language changed to: {$lang}

# Errors
error-config = Configuration error: {$message}
error-flutter = Flutter CLI error: {$message}
error-io = IO error: {$message}
error-invalid-arg = Invalid argument: {$message}
error-unknown = Unknown error: {$message}

# Debug and warning messages
debug-flutter-doctor = Running Flutter doctor...
warn-android-missing = Android directory not found
warn-ios-missing = iOS directory not found

# Error messages
error-message = Error: {$message}
error-unknown-occurred = An unexpected error occurred
error-flutter-doctor-failed = Flutter doctor failed. Please check your Flutter installation.

# Configuration
config-current = Current configuration:
config-language = Language: {$lang}
config-show-not-implemented = Showing configuration value is not implemented yet
config-set-not-implemented = Setting configuration value is not implemented yet 