//
//  FirstViewController.swift
//  anonyMoose iOS
//
//  Created by Alex Toop on 31/10/2016.
//  Copyright © 2016 Alexander Toop. All rights reserved.
//

import UIKit

class FirstViewController: UIViewController {
	@IBOutlet var anonyMooseTitle: UILabel!
	@IBOutlet var ScrollView: UIScrollView!
	@IBOutlet var ImageView: UIImageView!
	
	
	override func viewDidLoad() {
		super.viewDidLoad()
		// Do any additional setup after loading the view, typically from a nib.
		


		
		let post1 = UILabel(frame: CGRect(x: 0, y: 0, width: 250, height: 50))
		post1.center = CGPoint(x: 185, y: 40)
		post1.textAlignment = .left
		post1.numberOfLines = 6
		post1.font = post1.font.withSize(12)
		post1.text = "This is an example of an anonomous post that someone " +
			"could have posted to be shown on our app. The maximum length I " +
			"plan to allow is to have around around 4/5 lines at the absolute" +
			" max. There will be some room for movement but we will see yet."
		self.ScrollView.addSubview(post1)
		
		let anonyMooseTitle = UILabel(frame:
			CGRect(x: 0, y: 0, width: 250, height: 50))
		anonyMooseTitle.center = CGPoint(x: 160, y: 37)
		anonyMooseTitle.textAlignment = .center
		anonyMooseTitle.numberOfLines = 1
		anonyMooseTitle.font = anonyMooseTitle.font.withSize(30)
		anonyMooseTitle.font = UIFont(name:"HelveticaNeue-Bold", size: 30)
		anonyMooseTitle.text = "anonyMoose"
		self.view.addSubview(anonyMooseTitle)
		
		let line = UIView(frame: CGRect(x: 50, y: 100, width: 500, height: 1))
		line.center = CGPoint(x: 185, y: 80)
		line.backgroundColor = .black
		self.ScrollView.addSubview(line)

		
		ScrollView.contentSize.height = 1000
	};

	override func didReceiveMemoryWarning() {
		super.didReceiveMemoryWarning()
		// Dispose of any resources that can be recreated.
		
		
	}


}

