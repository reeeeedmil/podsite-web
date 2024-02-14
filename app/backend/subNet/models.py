from django.db import models
from django.core.validators import int_list_validator
# Create your models here.


class Net(models.Model):
    parent = models.ForeignKey('Net',
                               on_delete=models.CASCADE,
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
        return f"id: {self.id}//net address: {self.network_address}"
