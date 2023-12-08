from django.db import models
from django.core.validators import int_list_validator
from django.contrib.auth.models import User
#Create your models here.

class Net(models.Model):
    author = models.ForeignKey(User,
                               on_delete=models.CASCADE,
                               )
    parent = models.ForeignKey('Net',
                               on_delete=models.CASCADE,
                               blank=True,
                               null=True,
                               )

    network_name = models.CharField(max_length=100,
                                    blank=True,
                                    null=True
                                    )
    description = models.TextField(max_length=400,
                                   blank=True,
                                   null=True,
                                   )

    network_address = models.CharField(max_length=15, 
                                       validators=[
                                           int_list_validator(sep='.')
                                           ]
                                       )
    broadcast = models.CharField(max_length=15, 
                                   validators=[
                                       int_list_validator(sep='.')
                                       ]
                                   )
    prefix = models.PositiveIntegerField()

    mask = models.CharField(max_length=15, 
                           validators=[
                               int_list_validator(sep='.')
                               ]
                           )
    wildcard = models.CharField(max_length=15, 
                                   validators=[
                                       int_list_validator(sep='.')
                                       ]
                                   )

    

    def __str__(self):
        return f"author: {self.author.username}, ID: {self.id}, name: {self.network_name}, net address: {self.network_address}"
