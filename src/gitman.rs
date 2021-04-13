// this file will be used to clone and push settings to a specified upsteam url using git

// When a user is using the app, they should be given the option to fetch their configs from their upstream url

// This url can be set using the --set-upstream option, if upstream isnt set when the user is using the app
// the application will complain.

// When the upstream is set the user can chose to sync their settings, either clone from upstream git url or
// push their local changes to git

// git management will be separate from the local settings directory
// The local folder will be in setman's .config directory.
// When we are doing git clones or pushes we create a temporary git repo in /tmp
// this is done to remove the need for a permanent git folder in the .config directory

// I might think about redoing the sync command. So instead of a general sync command i could
// do two separate commands. One for push syncs and one for clone syncs. They should probably still
// be application specific with the option to sync all

// i will need to think more about the above thoughts
