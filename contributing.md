# Contributing to {{Project-Name}}

We welcome everyone to contribute what they can. Whether you are brand new, just want to contribute a little bit, or want contribute a lot there is probably something you can help out with.

## Where to get help
The main way to get help is on our [discord server](https://discord.gg/uh69TdKfBD).  Though, this guide should help you get started. It may be slightly lengthy, but it designed for those who are new so please don't let length intimidate you.

## Code of Conduct
Please be nice to each other when interacting with others and follow our [code of conduct](<link to code of conduct>).

## How to contribute
If the code adds a feature that is not already present in an issue, you can create a new issue for the feature and add the pull request to it. If the code adds a feature that is not already present in an issue, you can create a new issue for the feature and add the pull request to it.

### Contributing by adding an issue
If you have found a bug and would like to report it or if you have a feature that you feel we should add, then we'd love it if you opened an issue! ❤️ Before you do, please search the other issues to avoid creating a duplicate issue. 
Submit a new issue just hit the issue button and a choice between two templates should appear. Then, follow along with the template you chose. If you don't know how to fill in all parts of the template go ahead and skip those parts. You can edit the issue later.

### Contributing through code
In order to contribute through code follow the steps below. Note that you don't need to be the best programmer to contribute. Our discord is open for questions

 1. **Pick a feature** you would like to add or a bug you would like to fix
	 - If you wish to contribute but what you want to fix/add is not already covered in an existing issue, please open a new issue
 2. **Discuss** the issue with the rest of the community
	 - Before you write any code, it is recommended that you discuss your intention to write the code on the issue you are attempting to edit
	 -  This helps to stop you from wasting your time duplicating the work of others that maybe working on the same issue; at the same time
	 - This step also allows you to get helpful pointers on the community on some problems they may encountered on similar issues
 3. **Fork** the repository
	 - A fork creates a copy of the code on your Github, so you can work on it separately from everyone else.
	 - You can learn more about forking [here](https://docs.github.com/en/get-started/quickstart/fork-a-repo)
 4. Ensure that you have **commit signing** enabled
	 - This ensures that the code you submit was committed by you and not someone else who claims to be you
	 - You can learn more about how to setup commit signing [here](https://www.freecodecamp.org/news/what-is-commit-signing-in-git/ "https://www.freecodecamp.org/news/what-is-commit-signing-in-git/")
	 - If you have already made some commits that you wish to put in a pull request without signing them, then you can follow [this guide](https://dev.to/jmarhee/signing-existing-commits-with-gpg-5b58) on how to fix that.
 5. **Clone** the repository to your your computer.
	 - This puts a copy of your fork on your computer so you can edit it
	 - You can learn more about cloning repositories [here](https://docs.github.com/en/repositories/creating-and-managing-repositories/cloning-a-repository)
 6. **Build** the project
	 - For a detailed look on how to build {{project-name}} look at our [README file](<link to readme>)
 7. **Start writing** the code you wanted
	 - Open up your favorite code editor and make the changes that you wanted to make to the repository
	 - Make sure to test your code with `{{test-code-cmd}}`
 8. **Write tests** for your code
	 - If you are adding a new feature you should write tests that ensure that if someone make changes to the code it cannot break your new feature without breaking the test
	 - If your code add a new feature you should also write at least one documentation test. The documentation test's purpose is to demonstrate and document how the use the API feature
	 - If your code fixes a bug you should write tests that ensure that if some makes code changes in the future the bug does not re-emerge without breaking tests
	 - {% if wasm %}Tests should be put in the test folder and in a test file relevant to the project you are working in {% end if %}
	 - {% if !wasm %}Tests should be put inside the test folder inside the source folder and in a test file relevant to the project you are working in {% end if %}
	 - For more information on how to write tests look at [this link](https://doc.rust-lang.org/book/ch11-01-writing-tests.html)
 9. Ensure that the code that you made follows our rust **coding guide lines**
	 - You can find our list of guidelines [here](<link to coding guidelines>)
	 - This is a courtesy to the programmers that come after you. The easier your code is to read, the easier it will be for the next person to make modifications.
	 - If you find it difficult to follow the guidelines or if the guidelines or unclear please reach out to us through our discord linked above, or you can just continue and leave a comment in the pull request stage.
 10. **Commit and Push** your code
	 - This sends your changes to your repository branch
	 - You can learn more about committing code [here](https://docs.github.com/en/desktop/contributing-and-collaborating-using-github-desktop/making-changes-in-a-branch/committing-and-reviewing-changes-to-your-project) and pushing it [here](https://docs.github.com/en/get-started/using-git/pushing-commits-to-a-remote-repository).
 11. The final step is to create **pull request** to our main branch 🥳🎉
	 - A pull request is how you merge the code you just worked so hard on with the code everyone else has access to
	 - Once you have submitted your pull request, we will review your code and check to make sure the code implements the feature or fixes the bug. We may leave some feedback and suggest edits. You can make the changes we suggest by committing more code to your fork.
	 - You can lean more about pull request [here](https://docs.github.com/en/pull-requests/collaborating-with-pull-requests/proposing-changes-to-your-work-with-pull-requests/about-pull-requests)
