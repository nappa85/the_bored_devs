# Why MD-Aggregator?

Well, the reply can be briefly reassumed in "because I can".

Obviously a simplier solution would have been a shared repo and a CI job to generate static HTML from it, it would also have been lighter for the end user, and some other advantages like previews and I don't know what else...

But then, with a shared repo, how do you manage it? Everyone pushing to master? You lose control over your own articles, other can move it around, edit them without you noticing, delete them, etc...<br/>
Obviously you're under version control, so you don't really lose nothing, but you can't manually check the repo every day to intercept something gone wrong.<br/>
At this point you could protect master branch and accept edits over pull requests, ok, but if anyone can accept pull requests we haven't solved anything, someone could accept it's own PR.<br/>
You could require more than one approval to merge the PR, perfect, maybe ask for everyone approval? Understandable, but now your edits depends on other people's free time, it's an hobby blog, people have their life to live...

As you can see it isn't a simple subject, probably I'm overcomplicating it, but maybe there is a simplier way.<br/>
Maybe it's simplier on this side and more complicated on the other, so we are simply moving the complexity from a side to another.

## How does MD-Aggregator works?

MD-Aggregator sources from a list of git repositories, could be GitHub or GitLab repos, public or private.<br/>
It needs an access token to access repo files and few other infos, depending on the platform, then once an hour it caches all repo file list.<br/>
When cache is renewed, also template cache is renewed, this way there is nothing to wait for on user request.

Template file is a mustache file and has access only to the list of folders and MarkDown files, it's up to template implementors decide what to do with it.<br/>
On default implementation, it is printed as an unordered list (`<UL>`) and, on page load, via javascript, it's quickly recombined as a tree.

The base idea is that minor logics can be moved on client side, via javascript, to light up server work without the user noticing any slowdown.

Another obstacle is article ownership. Only giving the article path, as it comes from page URL, can lead on disambiguation problems if two repos contains a file with the same path and name.<br/>
To avoid this, every GET request that doesn't contains the `owner` parameter will simply output the base template cache contents.<br/>
It's up to template implementors loading the requested page disambiguating between homonyms.

The same happens for relative paths inside articles, e.g. images coming from the same repo.<br/>
It's up to template implementors adding the `owner` parameter to the path.

When an article is requested, a timed cache is checked, if the article is not there, or the cache entry is expired, article body is retrieved from source repo and placed into the cache.<br/>
This way a big number of requests on the same articles in a short amount of time won't end up in a big number of request to source repo.
